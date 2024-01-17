use crate::{db, util};
use sqlx::sqlite::SqlitePool;
use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct AddArgs {
    /// Name of the kata to create an entry for
    name: String,
}

pub(crate) async fn run(args: AddArgs) -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        let result = db::insert_kata_name(&pool, args.name).await;
        println!("{:?}", result);
        Ok(())
    } else {
        Err("key location not found in TOML file".into())
    }
}
