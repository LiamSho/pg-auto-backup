use serde::{Deserialize, Serialize};

mod postgresql;

pub use postgresql::PostgreSQL;

#[derive(Debug, Serialize, Deserialize)]
pub struct Databases {
    pub postgresql: Vec<PostgreSQL>,
}
