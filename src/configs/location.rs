use crate::configs::storage::Local;
use crate::configs::storage::S3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Location {
    S3(S3),
    Local(Local),
}
