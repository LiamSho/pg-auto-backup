use crate::configs::Connection;
use crate::configs::Database;
use crate::configs::PgDump;
use log::debug;
use tokio::process::Command;

pub async fn dump_database(
    database: &Database,
    pg_dump: &PgDump,
    connection: &Connection,
    output: &str,
) -> Result<(), Option<i32>> {
    let format = match &database.format {
        Some(val) => val,
        None => &pg_dump.format,
    };

    let mut binding = Command::new(&pg_dump.binary_path);
    let cmd = binding
        .arg(format!("--host={host}", host = &connection.host))
        .arg(format!("--port={port}", port = &connection.port))
        .arg(format!(
            "--username={username}",
            username = &connection.user
        ))
        .arg(format!("--format={format}", format = format))
        .arg(format!("--file={output}", output = output));

    // --section={}
    apply_selective_args(cmd, "section", &pg_dump.sections);

    // --no-{} & --disable-{}
    apply_switch_args_join(cmd, &pg_dump.do_not_save, &database.do_not_save);
    apply_switch_args_join(cmd, &pg_dump.disable, &database.disable);

    // --clean & --create
    apply_switch_args_compare_join(cmd, vec!["clean"], true, &pg_dump.clean, &database.clean);
    apply_switch_args_compare_join(cmd, vec!["create"], true, &pg_dump.create, &database.create);

    // --table & --exclude-table & --schema & --exclude-schema
    apply_selective_args_some(cmd, "table", &database.include_tables);
    apply_selective_args_some(cmd, "exclude-table", &database.exclude_tables);
    apply_selective_args_some(cmd, "schema", &database.include_schemata);
    apply_selective_args_some(cmd, "exclude-schema", &database.exclude_schemata);

    // extra_args
    match &database.extra_args {
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

    cmd.arg(&database.name);

    let commands = format!("{:?}", cmd);
    debug!("Executing pg_dump command: {}", commands);

    cmd.env("PGPASSWORD", &connection.password);

    let output = cmd.output().await.expect("Failed to execute pg_dump");

    if output.status.success() {
        Ok(())
    } else {
        Err(output.status.code())
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
