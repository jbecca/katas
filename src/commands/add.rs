use crate::{db, util};
use sqlx::sqlite::SqlitePool;
use std::{error::Error, path::{Path, PathBuf}};

use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct AddArgs {
    /// Name of the kata to create an entry for
    name: String,

    /// Path for template directory
    path: PathBuf,
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

pub(crate) fn parse_rust_main(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let main_path = path.join("src/main.rs");
    let main_str = std::fs::read_to_string(main_path.as_path())?;

    println!("{:?}", main_str);
    Ok(main_str)

}

pub(crate) fn create_cargo_toml(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let toml_path = path.join("Cargo.toml");
    let toml_str = std::fs::read_to_string(toml_path.as_path())?;
    Ok(toml_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::commands::add::parse_rust_main;

    #[test]
    fn test_rust_main() {
        let p = PathBuf::from("/Users/jeffreybecca/projects/katas/testdir");
        assert_eq!(parse_rust_main(p).unwrap(), String::from("this is a string"));
    }

    #[test]
    fn test_cargo_toml() {
        let p = PathBuf::from("/Users/jeffreybecca/projects/katas/testdir");
        assert_eq!(create_cargo_toml(p).unwrap(), String::from("this is a string"));
    }
}
