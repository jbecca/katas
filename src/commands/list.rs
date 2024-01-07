use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
pub struct ListArgs {
    /// number of entries to find
    #[arg(long, short, default_value_t = 10)]
    number: u32,
}
pub(crate) fn run(options: ListArgs) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}
