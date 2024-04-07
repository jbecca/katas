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

fn new_bucket(difficulty: Difficulty, current_bucket: Difficulty) -> Difficulty {
    match (difficulty, current_bucket) {
        (Difficulty::Easy, Difficulty::VeryHard) => Difficulty::Medium,
        (Difficulty::Easy, _) => Difficulty::Easy,
        (Difficulty::Medium, Difficulty::VeryHard) => Difficulty::Hard,
        (Difficulty::Medium, _) => Difficulty::Medium,
        (Difficulty::Hard, Difficulty::Easy) => Difficulty::Medium,
        (Difficulty::Hard, Difficulty::Medium) => Difficulty::Hard,
        (Difficulty::Hard, _) => Difficulty::VeryHard,
        (Difficulty::VeryHard, Difficulty::Easy) => Difficulty::Hard,
        (Difficulty::VeryHard, _) => Difficulty::VeryHard,
    }
}

async fn log_kata(
    pool: &SqlitePool,
    kata_name: String,
    language: Language,
) -> Result<(), Box<dyn Error>> {
    let result = sqlx::query(
        r#"UPDATE status SET time = datetime()
        WHERE id = (
            SELECT id from katas
            WHERE name = $1 )
        AND language = ?2;"#,
    )
    .bind(kata_name.as_str())
    .bind(language.to_string())
    .execute(pool)
    .await?
    .rows_affected();

    println!("Rows updated 1: {}", result);
    if result == 0 {
        println!("tryng to insert into status");
        dbg!(kata_name.as_str());
        println!("{:?}", "test_kata_1");
        let insert_statement = sqlx::query(
            r#"INSERT into status (id, time, language)
               VALUES (
                (SELECT id from katas
                    WHERE name = $1 ),
                datetime("1970-01-01 00:00:00"),
                $2);"#,
        )
        .bind(kata_name.as_str())
        .bind(language.to_string())
        .execute(pool)
        .await?
        .rows_affected();
        println!("Rows updated 2: {}", insert_statement);
    };

    Ok(())
}

pub(crate) async fn run(options: LogArgs) -> Result<(), Box<dyn Error>> {
    let user_cfg = parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        log_kata(&pool, options.name, options.language).await?;
        pool.close().await;
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}
