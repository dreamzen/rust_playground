//! This is an exercise for implementing grep in Rust

use minigrep_xiangzeng::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_xiangzeng::run(config) {
        eprintln!("failed run: {}", e);
        process::exit(1);
    }
}
