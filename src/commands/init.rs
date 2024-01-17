use std::error::Error;

use sqlx::sqlite::SqlitePool;

use crate::util;

pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        lib_katas::db::setup_tables(&pool).await?;
        Ok(())
    } else {
        Err("key location not found in TOML file".into())
    }
}
