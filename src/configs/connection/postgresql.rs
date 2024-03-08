use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgreSQLConnection {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}