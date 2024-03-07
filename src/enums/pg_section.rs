use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PgSection {
    PreData,
    Data,
    PostData,
}

impl fmt::Display for PgSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PgSection::PreData => write!(f, "pre-data"),
            PgSection::Data => write!(f, "data"),
            PgSection::PostData => write!(f, "post-data"),
        }
    }
}
