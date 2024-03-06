use chrono::FixedOffset;
use cron::Schedule;
use log::{error, info};
use std::env;
use std::str::FromStr;
use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use traits::Storage;

mod configs;
mod enums;
mod job;
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

    let tz = FixedOffset::east_opt(cfg.timezone_offset * 3600).unwrap();

    let cron_expression = cfg.cron.as_str();
    info!("Cron expression: {}", cron_expression);
    let schedule = match Schedule::from_str(cron_expression) {
        Ok(s) => {
            let next_datetime = s.upcoming(tz).next().unwrap();
            info!("Next tick of the scheduler: {}", next_datetime);
            s
        }
        Err(err) => panic!("Error parsing cron expression: {}", err),
    };

    let mut sched = JobScheduler::new().await?;

    add_jobs(&mut sched, schedule, tz).await?;

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
        configs::Location::Azure(azure) => (*azure).preflight_check().await,
    }
}

async fn add_jobs(
    sched: &mut JobScheduler,
    schedule: Schedule,
    timezone_offset: FixedOffset,
) -> Result<uuid::Uuid, JobSchedulerError> {
    sched
        .add(Job::new_cron_job_async_tz(
            schedule,
            timezone_offset,
            |uuid, mut l| {
                Box::pin(async move {
                    job::database_backup().await;

                    let cfg = configs::INSTANCE.get().unwrap();
                    let tz = FixedOffset::east_opt(cfg.timezone_offset * 3600).unwrap();

                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => {
                            let ts = ts.with_timezone(&tz);
                            info!("Next tick for backup job: {}", ts)
                        }
                        _ => error!("Error getting next tick for backup job"),
                    }
                })
            },
        )?)
        .await
}
