mod pg_dump;

pub use pg_dump::PgDump;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub pg_dump: Option<PgDump>,
}
