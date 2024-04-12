use lib_katas::util;
use lib_katas::util::Difficulty;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Get the most_recent kata in database by last completed time
pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    trace!("Starting commands::practice::run");
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        let (kata_name, cg, main) = find_most_recent_kata(&pool).await?;
        if let Some(practice_path) = user_cfg["practice_location"].as_str() {
            setup_kata(kata_name, main, cg, practice_path.into())?
        } else {
            let temp_path = std::env::current_dir();
            setup_kata(kata_name, main, cg, temp_path?)?;
        }
        pool.close().await;
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}

async fn find_most_recent_kata(
    conn: &SqlitePool,
) -> Result<(String, String, String), Box<dyn Error>> {
    trace!("Starting commands::practice::find_most_recent_kata");
    let result = sqlx::query(
        r#"SELECT * from katas
        INNER JOIN attempts
        ON katas.id = attempts.id
        ORDER BY time
        ASC LIMIT 1;"#,
    )
    .fetch_one(conn)
    .await?;
    let kata_name = result.get::<String, &str>("name");
    let cargo = result.get::<String, &str>("cargo");
    let main = result.get::<String, &str>("main");
    Ok((kata_name, cargo, main))
}

fn new_bucket(difficulty: Difficulty, current_bucket: Difficulty) -> Difficulty {
    trace!("Starting commands::practice::new_bucket");
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

fn setup_kata(
    kata_name: String,
    main_string: String,
    cargo_string: String,
    path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    trace!("Starting commands::practice::setup_kata");
    let mut cwd = path.clone();
    cwd.push(kata_name);
    cwd.push("src");
    std::fs::create_dir_all(cwd.as_path())?;

    cwd.push("main.rs");
    let mut main = File::create(cwd.as_path())?;
    main.write_all(main_string.as_bytes())?;

    cwd.pop();
    cwd.pop();
    cwd.push("Cargo.toml");
    let mut cargo = File::create(cwd.as_path())?;
    cargo.write_all(cargo_string.as_bytes())?;

    Ok(())
}
