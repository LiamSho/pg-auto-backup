use tokio::process::Command;

use log::{error, info};

use crate::configs::PgDump;

mod dump;

pub use dump::dump_database;

pub async fn preflight_check(pg_dump: &PgDump) {
    info!(
        "Preflight check: get pg_dump version by running {} --version",
        &pg_dump.binary_path
    );

    let output = Command::new(&pg_dump.binary_path)
        .arg("--version")
        .output()
        .await
        .expect("Failed to execute pg_dump");

    let mut output_vec = output.stdout;
    output_vec.retain(|&c| c != 10 && c != 13);
    let output_message = String::from_utf8(output_vec);

    match output_message {
        Ok(val) => info!("Preflight check: pg_dump output: {}", val),
        Err(e) => error!("Preflight check: Failed to parse pg_dump output: {}", e),
    };

    match output.status.success() {
        true => info!("Preflight check: pg_dump binary found and version is ok"),
        false => {
            let err_message = String::from_utf8(output.stderr).unwrap_or("unknwon".to_string());
            panic!(
                "Failed to run pg_dump --version with error: {}",
                err_message
            );
        }
    }
}
