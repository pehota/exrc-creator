extern crate exrc_tool;

use exrc_tool::*;
use std::process;

fn main() {
    use std::env;

    let config = Config::new(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("{}", error);
        process::exit(1);
    }
}
