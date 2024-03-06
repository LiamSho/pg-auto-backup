use crate::enums::{Disable, DoNotSave, Format, Section};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PgDump {
    pub binary_path: String,
    pub format: Format,
    pub sections: Vec<Section>,
    pub clean: Option<bool>,
    pub create: Option<bool>,
    pub do_not_save: Option<Vec<DoNotSave>>,
    pub disable: Option<Vec<Disable>>,
    pub extra_args: Option<Vec<String>>,
}
