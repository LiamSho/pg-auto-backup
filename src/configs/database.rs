use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub include_schemata: Vec<String>,
    pub exclude_schemata: Vec<String>,
    pub include_tables: Vec<String>,
    pub exclude_tables: Vec<String>,
}
