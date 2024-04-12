use crate::util;
use clap::Parser;
use sqlx::SqlitePool;
use std::{error::Error, path::PathBuf};
use toml::Table;

#[derive(Parser, Debug)]
pub(crate) struct AddArgs {
    /// Path for template directory
    path: PathBuf,
}

fn read_rust_main(path: PathBuf) -> Result<String, Box<dyn Error>> {
    trace!("Starting commands::add::read_rust_main");
    let main_path = path.join("src/main.rs");
    let main_str = std::fs::read_to_string(main_path.as_path())?;
    Ok(main_str)
}

fn read_cargo_toml(path: PathBuf) -> Result<String, Box<dyn Error>> {
    trace!("Starting commands::add::read_cargo_toml");
    let toml_path = path.join("Cargo.toml");
    let toml_str = std::fs::read_to_string(toml_path.as_path())?;
    Ok(toml_str)
}

fn read_kata_toml(path: PathBuf) -> Result<toml::Table, Box<dyn Error>> {
    trace!("Starting commands::add::read_kata_toml");
    let file_path = path.join("config.toml");
    let kata_config = std::fs::read_to_string(file_path)?
        .parse::<Table>()
        .expect("file was not TOML parsible");
    Ok(kata_config)
}

pub(crate) async fn run(args: AddArgs) -> Result<(), Box<dyn Error>> {
    trace!("Starting commands::add::run");
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        let rust_main = read_rust_main(args.path.clone())?;
        let cargo_toml = read_cargo_toml(args.path.clone())?;
        let kata_cfg = read_kata_toml(args.path.clone())?;

        let result = sqlx::query(
            r#"
           INSERT into katas (name) VALUES ( ?1 );"#,
        )
        .bind(kata_cfg["name"].as_str().unwrap())
        .execute(&pool)
        .await?
        .rows_affected();
        println!("rows added (result) {:?}", result);

        let result2 = sqlx::query(
            r#"
            INSERT into rust (id, main, cargo)
            VALUES ((SELECT id from katas WHERE name = $1), ?2, ?3);"#,
        )
        .bind(kata_cfg["name"].as_str())
        .bind(rust_main)
        .bind(cargo_toml)
        .execute(&pool)
        .await?
        .rows_affected();
        println!("rows added {:?}", result2);

        pool.close().await;
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_main() {
        let p = PathBuf::from("/Users/jeffreybecca/projects/katas/testdir");
        assert_eq!(
            read_rust_main(p).unwrap(),
            String::from("fn main() {\n    println!(\"Hello, world!\");\n}\n")
        );
    }

    #[test]
    fn test_cargo_toml() {
        let p = PathBuf::from("/Users/jeffreybecca/projects/katas/testdir");
        assert_eq!(read_cargo_toml(p).unwrap(), String::from("[package]\nname = \"testdir\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\n"));
    }

    #[test]
    fn test_kata_cfg() {
        let p = PathBuf::from("/Users/jeffreybecca/projects/katas/testdir");
        let cfg = read_kata_toml(p).unwrap();
        assert_eq!(cfg["name"].as_str(), Some("test_kata_1"));
    }
}
