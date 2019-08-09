extern crate minigrep;
use std::env;
use minigrep::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem in parsing : {}", err);
        process::exit(1);
    });
    minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Application Error : {}", err);
        process::exit(1);
    });
}
