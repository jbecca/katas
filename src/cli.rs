use clap::Parser;

/// CLI for kata trainer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// List all available katas
    #[arg(short, long)]
    list: bool,
}

pub fn parse_cli_args() -> Args {
    Args::parse()
}
