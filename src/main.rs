use lib_katas::test;
mod cli;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::ConnectOptions;
use std::error::Error;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = cli::parse_cli_args();
    test();

    let conn = SqliteConnectOptions::from_str("sqlite://data.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .read_only(false)
        .connect()
        .await?;
    println!("Hello, world!");
    Ok(())
}
