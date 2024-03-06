use tokio::process::Command;

use log::info;

use crate::configs::PgDump;

mod dump;

pub use dump::dump_database;

pub async fn preflight_check(pg_dump: &PgDump) {
    info!(
        "Preflight check: get pg_dump version by running {} --version",
        &pg_dump.binary_path
    );

    let status = Command::new(&pg_dump.binary_path)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .await;

    match status {
        Ok(_) => info!("Preflight check: pg_dump binary found and version is ok"),
        Err(e) => panic!("Failed to run pg_dump --version: {}", e),
    };
}
