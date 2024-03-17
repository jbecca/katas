# katas
CLI helper tool for practicing custom katas. Currently has support for rust language only, but more are planned. 

This tool is currently in an alpha state, where I will aggressively make breaking changes to better fit my own personal use case. 

## Usage
Firstly, the environment variable `KATA_CFG` location must be set, which should be the path of a TOML file. This tells the program where to look for the main configuration information. This location can be anywhere, but I suggest putting it in `$XDG_HOME`. This file must contain the following lines:
```toml
db_location=<full path to sqlite database>
practice_location=<full path to folder to write katas to>
```
To initialize a database, run `katas init`.

## Creating katas
Create a new cargo binary package. In the `main.rs` file, create the functionality you want, and ensure tests will check that the kata runs correctly. Then delete all of the implementation details and enter either `todo!()` or `unimplemented!()` in their place. Next, add in a `config.toml` to the crate root that contains the necessary configuration details for that kata. At a minimum, the file must contain `kata_name`, which the rest of the tool will use to describe the kata. Once all of this is done, the kata can be added to the database with `katas add <PATH>` where path is the root of the newly added and configured crate. This will only look for a `main.rs` file and `Cargo.toml` for now. This means that all implementation should be added into the main file, but any dependencies needed can be added with `cargo add <dep>` and should be tracked properly.

## Practicing
Running `katas practice` will find the oldest dated kata and copy it to the practice directory. Implement what is needed to make `cargo test` pass in that crate, then run `katas log --name <name> --language <language>`. This will update the last completed time to the current time in UTC. 

## Seeing all katas
To see available katas, run `katas list`. By default, it will list 10 katas, but that number can be changed with a supplied `-n <number>` option. 

## Deleting katas
A kata can be deleted from the database with `katas delete <name>`.

