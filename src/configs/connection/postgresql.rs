use serde::{Deserialize, Serialize};

use crate::extension::string_or_env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgreSQLConnection {
    #[serde(deserialize_with = "string_or_env")]
    pub host: String,
    #[serde(deserialize_with = "string_or_env")]
    pub port: String,
    #[serde(deserialize_with = "string_or_env")]
    pub user: String,
    #[serde(deserialize_with = "string_or_env")]
    pub password: String,
}
