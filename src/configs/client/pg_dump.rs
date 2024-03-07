use crate::{
    enums::{PgDisable, PgDoNotSave, PgFormat, PgSection},
    traits::PreflightCheck,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgDump {
    pub binary_path: String,
    pub format: PgFormat,
    pub sections: Vec<PgSection>,
    pub clean: Option<bool>,
    pub create: Option<bool>,
    pub do_not_save: Option<Vec<PgDoNotSave>>,
    pub disable: Option<Vec<PgDisable>>,
    pub extra_args: Option<Vec<String>>,
}

impl PreflightCheck for PgDump {
    async fn preflight_check(&self) -> Result<(), String> {
        info!(
            "Preflight check: get pg_dump version by running {} --version",
            &self.binary_path
        );

        let output = Command::new(&self.binary_path)
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
            true => {
                info!("Preflight check: pg_dump binary found and version is ok");
                Ok(())
            }
            false => {
                let err_message = String::from_utf8(output.stderr).unwrap_or("unknwon".to_string());
                Err(format!(
                    "Failed to run pg_dump --version with error: {}",
                    err_message
                ))
            }
        }
    }
}
