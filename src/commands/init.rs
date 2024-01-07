use std::error::Error;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::str::FromStr;
use sqlx::ConnectOptions;

use crate::db;

use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct InitArgs {
    /// name of kata to update
    #[arg(short, long)]
    name: String,
}

pub(crate) async fn run(options: InitArgs) -> Result<(), Box<dyn Error>> {
    println!("kata name {}", options.name);
    let mut conn = SqliteConnectOptions::from_str("sqlite://data.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .read_only(false)
        .connect()
        .await?;
    db::setup_tables(&mut conn).await;

    Ok(())
}
