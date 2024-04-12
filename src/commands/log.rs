use std::error::Error;

use clap::Parser;
use lib_katas::util::{parse_config, Difficulty, Language};
use sqlx::sqlite::SqlitePool;

#[derive(Parser, Debug)]
pub(crate) struct LogArgs {
    /// name of kata to update
    #[arg(short, long)]
    name: String,

    /// language used for the kata
    #[arg(short, long, value_enum)]
    language: Language,

    /// difficulty for practice attempt
    #[arg(short, long, value_enum)]
    difficulty: Difficulty,
}


async fn log_kata(
    pool: &SqlitePool,
    kata_name: String,
    difficulty: Difficulty,
) -> Result<(), Box<dyn Error>> {
    println!("trying to insert into status");
    dbg!(kata_name.as_str());
    println!("{:?}", "test_kata_1");
    let insert_statement = sqlx::query(
        r#"INSERT into attempts (id, difficulty, time)
            VALUES (
            (SELECT id from katas
                WHERE name = $1 ),
            $2,
            datetime());"#,
    )
    .bind(kata_name.as_str())
    .bind(difficulty.to_string())
    .execute(pool)
    .await?
    .rows_affected();
    println!("Rows added: {}", insert_statement);

    Ok(())
}

pub(crate) async fn run(options: LogArgs) -> Result<(), Box<dyn Error>> {
    let user_cfg = parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        log_kata(&pool, options.name, options.difficulty).await?;
        pool.close().await;
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}
