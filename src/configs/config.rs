use crate::enums::{Format, Section};
use log::LevelFilter;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use super::{storage::Local, Connection, Database, Location, PgDump};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub temp_dir: String,
    pub cron: String,
    pub timezone_offset: i32,
    pub log_level: LevelFilter,
    pub storage: Location,
    pub databases: Vec<Database>,
    pub connection: Connection,
    pub pg_dump: PgDump,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            temp_dir: "/tmp/pg-auto-backup".to_string(),
            cron: "0 0 * * * *".to_string(),
            timezone_offset: 0,
            log_level: LevelFilter::Info,
            storage: Location::Local(Local {
                path: "/var/lib/pg-auto-backup".to_string(),
            }),
            databases: vec![],
            connection: Connection {
                host: "localhost".to_string(),
                port: 5432,
                user: "postgres".to_string(),
                password: "password".to_string(),
            },
            pg_dump: PgDump {
                binary_path: "/usr/bin/pg_dump".to_string(),
                format: Format::Plain,
                sections: vec![Section::PreData, Section::Data, Section::PostData],
                do_not_save: None,
                disable: None,
                clean: None,
                create: None,
                extra_args: None,
            },
        }
    }
}

pub static INSTANCE: OnceCell<Config> = OnceCell::new();

pub fn init_config(config_file_path: Option<String>) {
    let cfg = match config_file_path {
        Some(val) => {
            println!("Loading config from path defined in args: {}", val);
            confy::load_path(val)
        }
        None => match std::env::var("PG_AUTO_BACKUP_CONFIG") {
            Ok(val) => {
                println!("Loading config from path defined in env: {}", val);
                confy::load_path(val)
            }
            Err(_e) => {
                let default_location =
                    confy::get_configuration_file_path("pg_auto_backup", None).unwrap();
                println!(
                    "Loading config from default location: {}",
                    default_location.display()
                );
                confy::load("pg_auto_backup", None)
            }
        },
    };

    match cfg {
        Ok(val) => INSTANCE.set(val).expect("Set config failed"),
        Err(e) => panic!("Config error: {:?}", e),
    }
}
