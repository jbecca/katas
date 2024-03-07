use lib_katas::util;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

/// Get the oldest kata in database by last completed time
pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        let (kata_name, cg, main) = find_oldest_kata(&pool).await?;
        setup_kata(kata_name, main, cg)?;
        Ok(())
    } else {
        Err("key location not found in TOML file".into())
    }
}

async fn find_oldest_kata(conn: &SqlitePool) -> Result<(String, String, String), Box<dyn Error>> {
    let result = sqlx::query(
        r#"SELECT * from katas
        INNER JOIN status
        ON katas.id = status.id
        LEFT JOIN rust
        ON katas.id = rust.id
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

fn setup_kata(
    kata_name: String,
    main_string: String,
    cargo_string: String,
) -> Result<(), Box<dyn Error>> {
    let mut cwd = std::env::current_dir()?;
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
