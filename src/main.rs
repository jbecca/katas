use clap::{Parser, Subcommand};
use std::error::Error;

mod commands;
use lib_katas::util;
use lib_katas::db;

#[derive(Subcommand, Debug)]
enum SubCommands {
    /// List katas
    List(commands::list::ListArgs),
    /// Log a kata practice attempt
    Log(commands::log::LogArgs),
    /// Initialize the sqliteDB
    Init,
    /// Add new kata to local database
    Add(commands::add::AddArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, styles=util::get_style())]
/// Code kata manager
struct Args {
    #[command(subcommand)]
    command: SubCommands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = Args::parse();

    match args.command {
        SubCommands::List(options) => commands::list::run(options).await?,
        SubCommands::Log(options) => commands::log::run(options).await?,
        SubCommands::Init => commands::init::run().await?,
        SubCommands::Add(options) => commands::add::run(options).await?
    };

    Ok(())
}
