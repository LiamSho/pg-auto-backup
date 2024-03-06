use std::path::Path;

use log::{debug, error, info, trace};

use crate::{configs, pg, traits::Storage};

pub async fn database_backup() {
    debug!("Running backup job");

    let cfg = configs::INSTANCE.get().unwrap();
    let tz: chrono::prelude::FixedOffset =
        chrono::FixedOffset::east_opt(cfg.timezone_offset * 3600).unwrap();

    let mut handles = vec![];

    for db in &cfg.databases {
        let handle = tokio::spawn(async move {
            let file_ext = match &db.format {
                Some(val) => val.get_file_ext(),
                None => &cfg.pg_dump.format.get_file_ext(),
            };

            let file_name = format!(
                "backup-{}.{}",
                chrono::Utc::now()
                    .with_timezone(&tz)
                    .format("%Y-%m-%d-%H-%M-%S"),
                file_ext
            );
            let random_file_name = uuid::Uuid::new_v4().to_string();
            let local_temp_file =
                Path::new(&cfg.temp_dir).join(format!("{}.{}", random_file_name, file_ext));
            let local_temp_file = local_temp_file.as_path().to_str().unwrap();

            let result =
                pg::dump_database(db, &cfg.pg_dump, &cfg.connection, local_temp_file).await;

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
                        configs::Location::S3(s3) => {
                            (*s3).save_file(local_temp_file, &db.name, file_name).await
                        }
                        configs::Location::Local(local) => {
                            (*local)
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

    debug!("Backup job completed")
}
