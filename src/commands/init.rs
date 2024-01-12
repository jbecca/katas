use std::error::Error;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::str::FromStr;
use sqlx::ConnectOptions;

use crate::{db, util};

use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct InitArgs {
    /// This subcommand is not currently used
    #[arg(short, long)]
    name: String,
}

pub(crate) async fn run(options: InitArgs) -> Result<(), Box<dyn Error>> {
    println!("kata name {}", options.name);
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["location"].as_str()  {
        let mut conn = SqliteConnectOptions::from_str(&format!("sqlite://{loc}"))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .read_only(false)
            .connect()
            .await?;
        db::setup_tables(&mut conn).await;
        Ok(())
    } else {
        Err("key location not found in TOML file".into())
    }

}
