use crate::enums::{PgDisable, PgDoNotSave, PgFormat, PgSection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PgDump {
    pub binary_path: String,
    pub format: PgFormat,
    pub sections: Vec<PgSection>,
    pub clean: Option<bool>,
    pub create: Option<bool>,
    pub do_not_save: Option<Vec<PgDoNotSave>>,
    pub disable: Option<Vec<PgDisable>>,
    pub extra_args: Option<Vec<String>>,
}
