mod azure;
mod local;
mod s3;

pub use azure::Azure;
pub use local::Local;
pub use s3::S3;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Storage {
    S3(S3),
    Local(Local),
    Azure(Azure),
}
