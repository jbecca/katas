use crate::util;
use clap::Parser;
use lib_katas::db;
use sqlx::sqlite::SqlitePool;
use std::error::Error;

#[derive(Parser, Debug)]
pub struct ListArgs {
    /// number of entries to find
    #[arg(long, short, default_value_t = 10)]
    number: u32,
}
pub(crate) async fn run(options: ListArgs) -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        db::list_n_katas(&pool, &options.number).await;
        Ok(())
    } else {
        Err("key location not found in TOML file".into())
    }
}
