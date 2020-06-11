use std::env;
use std::process;

use minigrep_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep_lib::Config::new(&args).unwrap_or_else(|err| {
        println!("Encountered a problem while parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
