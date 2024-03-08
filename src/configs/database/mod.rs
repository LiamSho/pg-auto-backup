use serde::{Deserialize, Serialize};

mod postgresql;

pub use postgresql::PostgreSQL;
pub use postgresql::PostgreSQLDatabase;

use crate::traits::PreflightCheck;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub postgresql: Option<PostgreSQL>,
}

impl PreflightCheck for Database {
    async fn preflight_check(&self) -> Result<(), String> {
        let mut empty = true;

        let result = match &self.postgresql {
            Some(postgresql) => {
                empty = false;
                postgresql.preflight_check().await
            }
            None => Ok(()),
        };

        if empty {
            Err("No database configuration found".to_string())
        } else {
            result
        }
    }
}
