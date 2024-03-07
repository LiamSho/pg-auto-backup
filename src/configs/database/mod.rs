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
        match &self.postgresql {
            Some(postgresql) => postgresql.preflight_check().await,
            None => Ok(()),
        }
    }
}
