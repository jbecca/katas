use lib_katas::util;
use clap::Parser;
use lib_katas::db;
use sqlx::sqlite::SqlitePool;
use std::error::Error;

pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        let _list_result = db::get_kata(&pool).await?;
        Ok(())
    } else {
        Err("key location not found in TOML file".into())
    }
}
