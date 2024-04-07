use crate::util;
use clap::Parser;
use sqlx::SqlitePool;
use std::error::Error;

#[derive(Parser, Debug)]
pub(crate) struct DeleteArgs {
    /// name of kata to delete from database
    name: String,
}

pub(crate) async fn run(args: DeleteArgs) -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        let result = sqlx::query(
            r#"
           DELETE FROM katas
           WHERE name = ( $1 );"#,
        )
        .bind(args.name.clone())
        .execute(&pool)
        .await?
        .rows_affected();
        println!("rows deleted (result) {:?}", result);
        pool.close().await;
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}
