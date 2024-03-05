use std::path::Path;

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::traits::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Local {
    pub path: String,
}

impl Storage for Local {
    async fn preflight_check(&self) {
        info!("Preflight check: check local storage location ");

        let path = std::path::Path::new(&self.path);

        match path.exists() {
            true => match path.is_dir() {
                true => info!(
                    "Preflight check: local storage location {} exists",
                    &self.path
                ),
                false => panic!(
                    "Preflight check: local storage location {} is not a directory",
                    &self.path
                ),
            },
            false => match tokio::fs::create_dir_all(path).await {
                Ok(_) => info!(
                    "Preflight check: create local storage location {}",
                    &self.path
                ),
                Err(_) => panic!(
                    "Preflight check: cannot create local storage location {}",
                    &self.path
                ),
            },
        }
    }

    async fn save_file(&self, local_temp_file: &str, save_path: &str, file_name: String) {
        let save_path = Path::new(&self.path).join(save_path);
        if !save_path.exists() {
            std::fs::create_dir_all(&save_path).unwrap();
        }
        let save_path = save_path.join(file_name);
        let save_path = save_path.as_path();
        match tokio::fs::copy(local_temp_file, save_path).await {
            Ok(_) => info!(
                "Save file {} to {} success",
                local_temp_file,
                save_path.display()
            ),
            Err(_) => error!(
                "Save file {} to {} failed",
                local_temp_file,
                save_path.display()
            ),
        };
    }
}
