use std::path::Path;

use crate::configs;
use crate::configs::client::PgDump;
use crate::configs::connection::PostgreSQLConnection;
use crate::configs::database::{PostgreSQL, PostgreSQLDatabase};
use crate::enums::one_of;
use crate::traits::{Dump, DumpJob, Storage};
use log::{debug, error, info, trace};
use tokio::process::Command;

impl DumpJob for PostgreSQL {
    async fn dump_databases(&self) {
        debug!("Running backup job");

        let cfg = configs::INSTANCE.get().unwrap();
        let tz: chrono::prelude::FixedOffset =
            chrono::FixedOffset::east_opt(cfg.general.timezone_offset * 3600).unwrap();

        let mut handles = vec![];

        for database in &self.databases {
            let client = self.client.clone();
            let connection = self.connection.clone();

            let db = match database {
                one_of::OneOf::TypeA(val) => val.into(),
                one_of::OneOf::TypeB(val) => val.clone(),
            };

            let handle = tokio::spawn(async move {
                let file_ext = match &db.format {
                    Some(val) => val.get_file_ext(),
                    None => client.format.get_file_ext(),
                };

                let file_name = format!(
                    "backup-{}.{}",
                    chrono::Utc::now()
                        .with_timezone(&tz)
                        .format("%Y-%m-%d-%H-%M-%S"),
                    file_ext
                );
                let random_file_name = uuid::Uuid::new_v4().to_string();
                let local_temp_file = Path::new(&cfg.general.temp_dir)
                    .join(format!("{}.{}", random_file_name, file_ext));
                let local_temp_file = local_temp_file.as_path().to_str().unwrap();

                let result = db
                    .dump_database(&client, &connection, local_temp_file)
                    .await;

                let result: Result<String, ()> = match result {
                    Err(val) => {
                        error!(
                            "Error dumping database {}, pg_dump exit with {:?}",
                            db.name, val
                        );
                        Err(())
                    }
                    Ok(_) => {
                        trace!("Database {} dumped successfully", db.name);
                        match &cfg.storage {
                            configs::Storage::S3(s3) => {
                                (*s3).save_file(local_temp_file, &db.name, file_name).await
                            }
                            configs::Storage::Local(local) => {
                                (*local)
                                    .save_file(local_temp_file, &db.name, file_name)
                                    .await
                            }
                            configs::Storage::Azure(azure) => {
                                (*azure)
                                    .save_file(local_temp_file, &db.name, file_name)
                                    .await
                            }
                        }
                    }
                };

                match tokio::fs::remove_file(local_temp_file).await {
                    Ok(_) => trace!("Remove local temp file: {}", local_temp_file),
                    Err(err) => error!("Error removing local temp file: {}", err),
                };

                match result {
                    Ok(val) => info!(
                        "Backup database {} successfully, saved to {}",
                        &db.name, val
                    ),
                    Err(_) => error!("Backup database {} failed", &db.name),
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}

impl Dump<PgDump, PostgreSQLConnection> for PostgreSQLDatabase {
    async fn dump_database(
        &self,
        pg_dump: &PgDump,
        connection: &PostgreSQLConnection,
        output: &str,
    ) -> Result<(), Option<i32>> {
        let format = match &self.format {
            Some(val) => val,
            None => &pg_dump.format,
        };

        let conn = &connection.parse();

        let mut binding = Command::new(&pg_dump.binary_path);
        let cmd = binding
            .arg(format!("--host={host}", host = &conn.host))
            .arg(format!("--port={port}", port = &conn.port))
            .arg(format!("--username={username}", username = &conn.user))
            .arg(format!("--format={format}", format = format))
            .arg(format!("--file={output}", output = output));

        // --section={}
        apply_selective_args(cmd, "section", &pg_dump.sections);

        // --no-{} & --disable-{}
        apply_switch_args_join(cmd, &pg_dump.do_not_save, &self.do_not_save);
        apply_switch_args_join(cmd, &pg_dump.disable, &self.disable);

        // --clean & --create
        apply_switch_args_compare_join(cmd, vec!["clean"], true, &pg_dump.clean, &self.clean);
        apply_switch_args_compare_join(cmd, vec!["create"], true, &pg_dump.create, &self.create);

        // --table & --exclude-table & --schema & --exclude-schema
        apply_selective_args_some(cmd, "table", &self.include_tables);
        apply_selective_args_some(cmd, "exclude-table", &self.exclude_tables);
        apply_selective_args_some(cmd, "schema", &self.include_schemata);
        apply_selective_args_some(cmd, "exclude-schema", &self.exclude_schemata);

        // role
        match &self.role {
            Some(val) => {
                cmd.arg(format!("--role={role}", role = val));
            }
            None => match &pg_dump.role {
                Some(val) => {
                    cmd.arg(format!("--role={role}", role = val));
                }
                None => (),
            },
        }

        // extra_args
        match &self.extra_args {
            Some(val) => {
                for arg in val {
                    cmd.arg(arg);
                }
            }
            None => match &pg_dump.extra_args {
                Some(val) => {
                    for arg in val {
                        cmd.arg(arg);
                    }
                }
                None => (),
            },
        }

        cmd.arg(&self.name);

        let commands = format!("{:?}", cmd);
        trace!("Executing pg_dump command: {}", commands);

        cmd.env("PGPASSWORD", &conn.password);

        let output = cmd.output().await.expect("Failed to execute pg_dump");

        match output.status.success() {
            true => Ok(()),
            false => Err(output.status.code()),
        }
    }
}

fn apply_switch_args<T>(cmd: &mut Command, values: &[T])
where
    T: std::fmt::Display,
{
    for value in values {
        cmd.arg(format!("--{val}", val = value));
    }
}

fn apply_switch_args_join<T>(
    cmd: &mut Command,
    global_value: &Option<Vec<T>>,
    override_value: &Option<Vec<T>>,
) where
    T: std::fmt::Display,
{
    match override_value {
        Some(val) => apply_switch_args(cmd, &val),
        None => match global_value {
            Some(val) => apply_switch_args(cmd, &val),
            None => (),
        },
    }
}

fn apply_switch_args_compare_join<T, K>(
    cmd: &mut Command,
    options: Vec<K>,
    append_value: T,
    global_value: &Option<T>,
    override_value: &Option<T>,
) where
    T: PartialEq,
    K: std::fmt::Display,
{
    match override_value {
        Some(val) => {
            if *val == append_value {
                apply_switch_args(cmd, &options);
            }
        }
        None => match global_value {
            Some(val) => {
                if *val == append_value {
                    apply_switch_args(cmd, &options);
                }
            }
            None => (),
        },
    }
}

fn apply_selective_args<T>(cmd: &mut Command, option: &str, values: &[T])
where
    T: std::fmt::Display,
{
    for value in values {
        cmd.arg(format!("--{opt}={val}", opt = option, val = value));
    }
}

fn apply_selective_args_some<T>(cmd: &mut Command, option: &str, values: &Option<Vec<T>>)
where
    T: std::fmt::Display,
{
    if let Some(values) = values {
        apply_selective_args(cmd, option, values)
    }
}
