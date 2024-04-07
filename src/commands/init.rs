use std::error::Error;

use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{Sqlite, SqlitePool},
};

use lib_katas::util;

pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        if !Sqlite::database_exists(&format!("sqlite://{loc}"))
            .await
            .unwrap_or(false)
        {
            match Sqlite::create_database(&format!("sqlite://{loc}")).await {
                Ok(_) => {
                    println!("DB created");
                    let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
                    lib_katas::db::setup_tables(&pool).await?;
                    pool.close().await;
                }
                Err(err) => panic!("error: {}", err),
            }
        } else {
            let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
            lib_katas::db::setup_tables(&pool).await?;
            pool.close().await;
        }
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}
