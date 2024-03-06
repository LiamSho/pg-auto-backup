use serde::{Deserialize, Serialize};

use crate::enums::{Disable, DoNotSave, Format, Section};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub format: Option<Format>,
    pub include_schemata: Option<Vec<String>>,
    pub exclude_schemata: Option<Vec<String>>,
    pub include_tables: Option<Vec<String>>,
    pub exclude_tables: Option<Vec<String>>,
    pub clean: Option<bool>,
    pub create: Option<bool>,
    pub sections: Option<Vec<Section>>,
    pub do_not_save: Option<Vec<DoNotSave>>,
    pub disable: Option<Vec<Disable>>,
    pub extra_args: Option<Vec<String>>,
}
