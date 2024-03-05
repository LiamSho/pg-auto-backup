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
            DoNotSave::LargeObjects => write!(f, "large-objects"),
            DoNotSave::Owner => write!(f, "owner"),
            DoNotSave::Privileges => write!(f, "privileges"),
            DoNotSave::Comments => write!(f, "comments"),
            DoNotSave::Publications => write!(f, "publications"),
            DoNotSave::SecurityLabels => write!(f, "security-labels"),
            DoNotSave::Subscriptions => write!(f, "subscriptions"),
            DoNotSave::TableAccessMethod => write!(f, "table-access-method"),
            DoNotSave::Tablespaces => write!(f, "tablespaces"),
            DoNotSave::ToastCompression => write!(f, "toast-compression"),
            DoNotSave::UnloggedTableData => write!(f, "unlogged-table-data"),
        }
    }
}
