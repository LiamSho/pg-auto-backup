use crate::{configs, traits::DumpJob};

pub async fn database_backup() {
    let cfg = configs::INSTANCE.get().unwrap();

    match &cfg.database.postgresql {
        Some(val) => {
            val.dump_databases().await;
        }
        None => (),
    }
}
