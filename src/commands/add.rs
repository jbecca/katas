use std::error::Error;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::str::FromStr;
use sqlx::ConnectOptions;

use crate::{db, util};

use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct AddArgs {
    /// Name of the kata to create an entry for
    name: String
}

pub(crate) async fn run(args: AddArgs) -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["location"].as_str()  {
        let mut conn = SqliteConnectOptions::from_str(&format!("sqlite://{loc}"))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .read_only(false)
            .connect()
            .await?;
        let result = db::insert_kata_name(&mut conn, args.name).await;
        println!("{:?}", result);
        Ok(())
    } else {
        Err("key location not found in TOML file".into())
    }

}
