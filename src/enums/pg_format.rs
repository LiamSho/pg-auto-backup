use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum PgFormat {
    Plain,
    Custom,
    Tar,
}

impl fmt::Display for PgFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PgFormat::Custom => write!(f, "custom"),
            PgFormat::Tar => write!(f, "tar"),
            PgFormat::Plain => write!(f, "plain"),
        }
    }
}

impl PgFormat {
    pub fn get_file_ext(&self) -> &str {
        match &self {
            PgFormat::Plain => "sql",
            PgFormat::Custom => "bin",
            PgFormat::Tar => "tar",
        }
    }
}
