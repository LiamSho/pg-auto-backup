use chrono::Utc;
use cron::Schedule;
use log::{error, info};
use std::str::FromStr;
use std::{env, path::Path};
use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use traits::Storage;

mod configs;
mod enums;
mod pg;
mod traits;

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    let args: Vec<String> = env::args().collect();
    if let Some(index) = args.iter().position(|arg| arg == "-c") {
        let config_file = &args[index + 1];
        configs::init_config(Some(config_file.to_string()));
    } else {
        configs::init_config(None);
    }

    let cfg = configs::INSTANCE.get().unwrap();

    env_logger::Builder::new()
        .filter_level(cfg.log_level)
        .init();

    let app_version = env!("CARGO_PKG_VERSION");
    info!("Starting pg-auto-backup {}", app_version);

    preflight_check().await;

    let cron_expression = cfg.cron.as_str();
    info!("Cron expression: {}", cron_expression);
    let schedule = match Schedule::from_str(cron_expression) {
        Ok(s) => {
            let next_datetime = s.upcoming(Utc).next().unwrap();
            info!("Next tick of the scheduler: {}", next_datetime);

            s
        }
        Err(err) => panic!("Error parsing cron expression: {}", err),
    };

    let mut sched = JobScheduler::new().await?;

    sched
        .add(Job::new_cron_job_async(schedule, |uuid, mut l| {
            Box::pin(async move {
                run_job().await;

                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => info!("Next tick for backup job: {}", ts),
                    _ => error!("Error getting next tick for backup job"),
                }
            })
        })?)
        .await?;

    info!("Starting scheduler");
    sched.start().await?;

    match signal::ctrl_c().await {
        Ok(()) => info!("Received SIGINT"),
        Err(err) => panic!("Error listening for signal: {}", err),
    };

    info!("Shutting down scheduler");
    sched.shutdown().await
}

async fn preflight_check() {
    let cfg = configs::INSTANCE.get().unwrap();

    pg::preflight_check(&cfg.pg_dump).await;

    match &cfg.storage {
        configs::Location::S3(s3) => (*s3).preflight_check().await,
        configs::Location::Local(local) => (*local).preflight_check().await,
    }
}

async fn run_job() {
    info!("Running backup job");

    let cfg = configs::INSTANCE.get().unwrap();
    let file_ext = (*cfg).pg_dump.get_file_ext();

    let mut handles = vec![];

    for db in &cfg.databases {
        let handle = tokio::spawn(async move {
            let file_name = format!(
                "backup-{}.{}",
                chrono::Utc::now().format("%Y-%m-%d-%H-%M-%S"),
                file_ext
            );
            let random_file_name = uuid::Uuid::new_v4().to_string();
            let local_temp_file = Path::new(&cfg.temp_dir).join(random_file_name);
            let local_temp_file = local_temp_file.as_path().to_str().unwrap();

            let result =
                pg::dump_database(db, &cfg.pg_dump, &cfg.connection, local_temp_file).await;

            match result {
                Err(val) => {
                    error!(
                        "Error dumping database {}, pg_dump exit with {:?}",
                        db.name, val
                    );
                }
                Ok(_) => {
                    info!("Backup of database {} successed", db.name);
                    match &cfg.storage {
                        configs::Location::S3(s3) => {
                            (*s3).save_file(local_temp_file, &db.name, file_name).await;
                        }
                        configs::Location::Local(local) => {
                            (*local)
                                .save_file(local_temp_file, &db.name, file_name)
                                .await;
                        }
                    };
                }
            }

            match tokio::fs::remove_file(local_temp_file).await {
                Ok(_) => info!("Remove local temp file: {}", local_temp_file),
                Err(err) => error!("Error removing local temp file: {}", err),
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    info!("Backup job completed")
}
