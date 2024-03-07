use serde::{Deserialize, Serialize};

use crate::{
    configs::{client::PgDump, connection::PostgreSQLConnection},
    enums::{one_of::OneOf, PgDisable, PgDoNotSave, PgFormat, PgSection},
    traits::PreflightCheck,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostgreSQL {
    pub databases: Vec<OneOf<String, PostgreSQLDatabase>>,
    pub connection: PostgreSQLConnection,
    pub client: PgDump,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgreSQLDatabase {
    pub name: String,
    pub format: Option<PgFormat>,
    pub include_schemata: Option<Vec<String>>,
    pub exclude_schemata: Option<Vec<String>>,
    pub include_tables: Option<Vec<String>>,
    pub exclude_tables: Option<Vec<String>>,
    pub clean: Option<bool>,
    pub create: Option<bool>,
    pub sections: Option<Vec<PgSection>>,
    pub do_not_save: Option<Vec<PgDoNotSave>>,
    pub disable: Option<Vec<PgDisable>>,
    pub extra_args: Option<Vec<String>>,
}

impl From<&String> for PostgreSQLDatabase {
    fn from(val: &String) -> Self {
        PostgreSQLDatabase {
            name: val.clone(),
            format: None,
            include_schemata: None,
            exclude_schemata: None,
            include_tables: None,
            exclude_tables: None,
            clean: None,
            create: None,
            sections: None,
            do_not_save: None,
            disable: None,
            extra_args: None,
        }
    }
}

impl PreflightCheck for PostgreSQL {
    async fn preflight_check(&self) -> Result<(), String> {
        self.client.preflight_check().await
    }
}
