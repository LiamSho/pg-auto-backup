mod postgresql;

pub use postgresql::PostgreSQLConnection;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub postgresql: Option<PostgreSQLConnection>,
}
