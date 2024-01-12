use clap::ValueEnum;
use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use toml::Table;
use std::env;
use std::error::Error;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub(crate) enum Status {
    Success,
    Failure,
}

pub fn get_style() -> clap::builder::Styles {
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default());
    styles
}

/// currently, uses env variable KATA_CFG to find user config file location
pub fn parse_config() -> Result<toml::Table, Box<dyn Error>> {
    let cfg_loc = env::var("KATA_CFG").expect("KATA_CFG environment variable should be set by the user");
    let value = std::fs::read_to_string(cfg_loc)?.parse::<Table>().expect("file should be TOML parsible");

    Ok( value )
}
