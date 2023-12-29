use lib_katas::test;
mod cli;
pub fn main() {
    let args = cli::parse_cli_args();
    test();
    println!("Hello, world!");
}
