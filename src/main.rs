use std::env;
use std::process;

use grape::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grape::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    };
}
