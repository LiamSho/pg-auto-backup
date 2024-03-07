use serde::{Deserialize, Serialize};

use crate::enums::{PgDisable, PgDoNotSave, PgFormat, PgSection};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostgreSQL {
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
