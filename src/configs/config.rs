use crate::enums::{PgFormat, PgSection};
use log::LevelFilter;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use super::{
    client::PgDump, connection::PostgreSQLConnection, storage::Local, Client, Connection,
    Databases, General, Storage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: General,
    pub storage: Storage,
    pub databases: Databases,
    pub connection: Connection,
    pub client: Client,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: General {
                temp_dir: "/tmp/pg-auto-backup".to_string(),
                run_at_start: false,
                cron: "0 0 * * * *".to_string(),
                timezone_offset: 0,
                log_level: LevelFilter::Info,
            },
            storage: Storage::Local(Local {
                path: "/var/lib/pg-auto-backup".to_string(),
            }),
            databases: Databases {
                postgresql: Vec::new(),
            },
            connection: Connection {
                postgresql: Some(PostgreSQLConnection {
                    host: "localhost".to_string(),
                    port: 5432,
                    user: "postgres".to_string(),
                    password: "password".to_string(),
                }),
            },
            client: Client {
                pg_dump: Some(PgDump {
                    binary_path: "/usr/bin/pg_dump".to_string(),
                    format: PgFormat::Plain,
                    sections: vec![PgSection::PreData, PgSection::Data, PgSection::PostData],
                    do_not_save: None,
                    disable: None,
                    clean: None,
                    create: None,
                    extra_args: None,
                }),
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
