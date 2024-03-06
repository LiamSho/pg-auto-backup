use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum DoNotSave {
    LargeObjects,
    Owner,
    Privileges,
    Comments,
    Publications,
    SecurityLabels,
    Subscriptions,
    TableAccessMethod,
    Tablespaces,
    ToastCompression,
    UnloggedTableData,
}

impl fmt::Display for DoNotSave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DoNotSave::LargeObjects => write!(f, "no-large-objects"),
            DoNotSave::Owner => write!(f, "no-owner"),
            DoNotSave::Privileges => write!(f, "no-privileges"),
            DoNotSave::Comments => write!(f, "no-comments"),
            DoNotSave::Publications => write!(f, "no-publications"),
            DoNotSave::SecurityLabels => write!(f, "no-security-labels"),
            DoNotSave::Subscriptions => write!(f, "no-subscriptions"),
            DoNotSave::TableAccessMethod => write!(f, "no-table-access-method"),
            DoNotSave::Tablespaces => write!(f, "no-tablespaces"),
            DoNotSave::ToastCompression => write!(f, "no-toast-compression"),
            DoNotSave::UnloggedTableData => write!(f, "no-unlogged-table-data"),
        }
    }
}
