use clap::{Parser, Subcommand};
use std::error::Error;

mod commands;
mod util;
mod db;

#[derive(Subcommand, Debug)]
enum SubCommands {
    List(commands::list::ListArgs),
    Log(commands::log::LogArgs),
    Init(commands::init::InitArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: SubCommands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = Args::parse();

    match args.command {
        SubCommands::List(options) => commands::list::run(options)?,
        SubCommands::Log(options) => commands::log::run(options)?,
        SubCommands::Init(options) => commands::init::run(options).await?,
    };

    Ok(())
}
