use clap::Parser;
use lib_katas::util;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::error::Error;

#[derive(Parser, Debug)]
pub struct ListArgs {
    /// number of entries to find
    #[arg(long, short, default_value_t = 10)]
    number: u32,
}
pub(crate) async fn run(options: ListArgs) -> Result<(), Box<dyn Error>> {
    trace!("Starting commands::list::run");
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        list_n_katas(&pool, &options.number).await?;
        pool.close().await;
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}

async fn list_n_katas(conn: &SqlitePool, number: &u32) -> Result<(), Box<dyn Error>> {
    let results = sqlx::query(
        r#"SELECT * from katas INNER JOIN status on katas.id = status.id LIMIT ?1;"#,
    )
    .bind(number.to_string())
    .fetch_all(conn)
    .await?;

    println!(
        "{:>5} {:>24} {:>24} {:>22}",
        "entry", "name", "due", "successful completions"
    );
    for (idx, row) in results.iter().enumerate() {
        println!(
            "{:>5} {:>24} {:>24} {:>22}",
            idx,
            row.get::<String, &str>("name"),
            row.get::<String, &str>("due"),
            row.get::<i32, &str>("n_success")
        );
    }

    Ok(())
}
