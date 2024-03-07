use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum PgDisable {
    DollarQuoting,
    Triggers,
}

impl fmt::Display for PgDisable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PgDisable::DollarQuoting => write!(f, "disable-dollar-quoting"),
            PgDisable::Triggers => write!(f, "disable-triggers"),
        }
    }
}
