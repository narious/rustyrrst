use std::env;
use std::process;

use minigrep::Config;

// Main job for argument collecting and error handling
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1)
    });


    // Because run doesn't actually return a value we would want ot unwrap we can use if let to handle errors
    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    }
}
