use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::ValueEnum;
use std::env;
use std::error::Error;
use toml::Table;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Language {
    Rust = 1,
    Python = 2,
    C = 3,
    Lua = 4,
}

pub fn get_style() -> clap::builder::Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

/// currently, uses env variable KATA_CFG to find user config file location
pub fn parse_config() -> Result<toml::Table, Box<dyn Error>> {
    let cfg_loc =
        env::var("KATA_CFG").expect("KATA_CFG environment variable should be set by the user");
    let value = std::fs::read_to_string(cfg_loc)?
        .parse::<Table>()
        .expect("file should be TOML parsible");

    Ok(value)
}
