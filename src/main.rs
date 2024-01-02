use lib_katas::test;
mod cli;
mod db;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::ConnectOptions;
use std::error::Error;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = cli::parse_cli_args();
    test();

    let mut conn = SqliteConnectOptions::from_str("sqlite://data.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .read_only(false)
        .connect()
        .await?;
    println!("Hello, world!");
    db::setup_tables(&mut conn).await;

    Ok(())
}
