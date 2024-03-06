use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    Plain,
    Custom,
    Tar,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Format::Custom => write!(f, "custom"),
            Format::Tar => write!(f, "tar"),
            Format::Plain => write!(f, "plain"),
        }
    }
}

impl Format {
    pub fn get_file_ext(&self) -> &str {
        match &self {
            Format::Plain => "sql",
            Format::Custom => "bin",
            Format::Tar => "tar",
        }
    }
}
