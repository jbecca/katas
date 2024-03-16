use std::error::Error;

use clap::Parser;
use lib_katas::util::{parse_config, Language};
use sqlx::sqlite::SqlitePool;

#[derive(Parser, Debug)]
pub(crate) struct LogArgs {
    /// name of kata to update
    #[arg(short, long)]
    name: String,

    /// language used for the kata
    #[arg(short, long, value_enum)]
    language: Language,
}

pub(crate) async fn run(options: LogArgs) -> Result<(), Box<dyn Error>> {
    let user_cfg = parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        lib_katas::db::log_kata(&pool, options.name, options.language).await?;
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}
