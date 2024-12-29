use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Using iterators we can pass in the iterator directly to build instead of consuming it using collect()
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1)
    });


    // Because run doesn't actually return a value we would want ot unwrap we can use if let to handle errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
