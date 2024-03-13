use crate::{db, util};
use lib_katas::util::Language;
use sqlx::types::Text;
use sqlx::{Row, SqlitePool};
use std::{
    error::Error,
    path::{Path, PathBuf},
};
use toml::Table;

use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct DeleteArgs {
    /// name of kata to delete from database
    name: String,
}

pub(crate) async fn run(args: DeleteArgs) -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["location"].as_str() {
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
        Ok(())

    } else {
        Err("key location not found in TOML file".into())
    }
}
