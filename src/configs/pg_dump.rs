use crate::enums::{DoNotSave, Format, Section};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PgDump {
    pub binary_path: String,
    pub format: Format,
    pub sections: Vec<Section>,
    pub do_not_save: Vec<DoNotSave>,
}

impl PgDump {
    pub fn get_file_ext(&self) -> &str {
        match &self.format {
            Format::Plain => "sql",
            Format::Custom => "bin",
            Format::Tar => "tar",
        }
    }
}
