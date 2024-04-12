extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::{Parser, Subcommand};
use std::error::Error;

mod commands;
use lib_katas::util;

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
    /// Get kata that needs practice
    Practice,
    /// Delete a kata from the database
    Delete(commands::delete::DeleteArgs),
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
    pretty_env_logger::init();
    trace!("Entering main");
    trace!("parsing args");
    let args = Args::parse();

    trace!("Matching command");
    match args.command {
        SubCommands::List(options) => commands::list::run(options).await?,
        SubCommands::Log(options) => commands::log::run(options).await?,
        SubCommands::Init => commands::init::run().await?,
        SubCommands::Add(options) => commands::add::run(options).await?,
        SubCommands::Practice => commands::practice::run().await?,
        SubCommands::Delete(options) => commands::delete::run(options).await?,
    };

    Ok(())
}
