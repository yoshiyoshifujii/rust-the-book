extern crate minigrep_refactor;

use std::{env, process};

use minigrep_refactor::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_refactor::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
