use crate::configs::Connection;
use crate::configs::Database;
use crate::configs::PgDump;
use log::{error, info};
use tokio::process::Command;

pub async fn dump_database(
    database: &Database,
    pg_dump: &PgDump,
    connection: &Connection,
    output: &str,
) {
    let mut binding = Command::new(&pg_dump.binary_path);
    let cmd = binding
        .env("PGPASSWORD", &connection.password)
        .arg(format!("--host={host}", host = &connection.host))
        .arg(format!("--port={port}", port = &connection.port))
        .arg(format!(
            "--username={username}",
            username = &connection.user
        ))
        .arg(format!("--format={format}", format = &pg_dump.format))
        .arg(format!("--file={output}", output = output));

    for do_not_save in &pg_dump.do_not_save {
        cmd.arg(format!("--no-{do_not_save}", do_not_save = do_not_save));
    }

    apply_selective_args(cmd, "section", &pg_dump.sections);
    apply_selective_args(cmd, "table", &database.include_tables);
    apply_selective_args(cmd, "exclude-table", &database.exclude_tables);
    apply_selective_args(cmd, "schema", &database.include_schemata);
    apply_selective_args(cmd, "exclude-schema", &database.exclude_schemata);

    cmd.arg(&database.name);

    let output = cmd.output().await.expect("Failed to execute pg_dump");

    if output.status.success() {
        info!("Backup of database {} was successful", &database.name);
    } else {
        error!("Backup of database {} failed", &database.name);
    }
}

fn apply_selective_args<T>(cmd: &mut Command, option: &str, values: &[T])
where
    T: std::fmt::Display,
{
    for value in values {
        cmd.arg(format!("--{opt}={val}", opt = option, val = value));
    }
}
