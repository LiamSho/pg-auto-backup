use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PgDoNotSave {
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

impl fmt::Display for PgDoNotSave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PgDoNotSave::LargeObjects => write!(f, "no-large-objects"),
            PgDoNotSave::Owner => write!(f, "no-owner"),
            PgDoNotSave::Privileges => write!(f, "no-privileges"),
            PgDoNotSave::Comments => write!(f, "no-comments"),
            PgDoNotSave::Publications => write!(f, "no-publications"),
            PgDoNotSave::SecurityLabels => write!(f, "no-security-labels"),
            PgDoNotSave::Subscriptions => write!(f, "no-subscriptions"),
            PgDoNotSave::TableAccessMethod => write!(f, "no-table-access-method"),
            PgDoNotSave::Tablespaces => write!(f, "no-tablespaces"),
            PgDoNotSave::ToastCompression => write!(f, "no-toast-compression"),
            PgDoNotSave::UnloggedTableData => write!(f, "no-unlogged-table-data"),
        }
    }
}
