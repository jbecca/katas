use clap::{Parser,Subcommand};
use lib_katas::test;
mod cli;
mod db;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::ConnectOptions;
use std::error::Error;
use std::str::FromStr;

mod commands;
mod util;

#[derive(Subcommand, Debug)]
enum SubCommands {
    List(commands::list::ListArgs),
    Log(commands::log::LogArgs)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: SubCommands
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = Args::parse();
    test();

    let mut conn = SqliteConnectOptions::from_str("sqlite://data.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .read_only(false)
        .connect()
        .await?;
    println!("Hello, world!");
    db::setup_tables(&mut conn).await;

    match args.command {
        SubCommands::List(options) => commands::list::run(options)?,
        SubCommands::Log(options) => commands::log::run(options)?
    };

    Ok(())
}
