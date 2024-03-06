use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum Disable {
    DollarQuoting,
    Triggers,
}

impl fmt::Display for Disable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Disable::DollarQuoting => write!(f, "disable-dollar-quoting"),
            Disable::Triggers => write!(f, "disable-triggers"),
        }
    }
}
