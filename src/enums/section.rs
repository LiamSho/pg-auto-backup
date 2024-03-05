use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum Section {
    PreData,
    Data,
    PostData,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Section::PreData => write!(f, "pre-data"),
            Section::Data => write!(f, "data"),
            Section::PostData => write!(f, "post-data"),
        }
    }
}
