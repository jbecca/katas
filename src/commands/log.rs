use std::error::Error;

use clap::{Parser};
use crate::util::Status;

#[derive(Parser, Debug)]
pub(crate) struct LogArgs {
    /// name of kata to update
    #[arg(short, long)]
    name: String,

    /// status of the kata attempt
    #[arg(short, long, value_enum)]
    status: Status,
}

pub(crate) fn run(options: LogArgs) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}
