use clap::Parser;
use lib_katas::test;
mod cli;
pub fn main() {
    let args = cli::Args::parse();
    test();
    println!("Hello, world!");
}
