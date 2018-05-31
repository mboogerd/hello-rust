// Bring the lib.rs contents into scope
extern crate ch12cli;

use std::env;
use std::process;
use ch12cli::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = ch12cli::run(config) {
        eprintln!("Application terminated with error: '{}'", e);
        process::exit(1);
    }
}