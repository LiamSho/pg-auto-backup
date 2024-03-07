use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub temp_dir: String,
    pub cron: String,
    pub timezone_offset: i32,
    pub run_at_start: bool,
    pub log_level: LevelFilter,
}
