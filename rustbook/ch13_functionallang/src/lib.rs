use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Change the arg type to Iterator<Item=String> instead of &[String]
    pub fn build(mut args: impl iterator<Item = String>) -> Result<Config, &'static str> {

        // Replacing the indexing with args.next() calls and matching
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        
        // Set environment variable on windows using $Env:IGNORE_CASE=1; cargo run -- <pattern> <file>
        // This will remain valid for that powershell instance
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
        
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{line}");
    }
    Ok(())
}

// In this case the return should reference the lifetime of the contents parameter
pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {

    // Using iterators and iterator adapters we can remove the need to define mutable 'results' vector to return
    // Also simplyfy the code
    contents.lines()
            .filter(|line| {
                if ignore_case {    
                    if line.to_lowercase().contains(query.to_lowercase().as_str()) {
                        return true
                    }
                } else {
                    if line.contains(query) {
                        return true
                    }
                }
                return false
            })
            .collect();
}


// TDD: Write a failing test, Write a test to pass it
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, false));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Trust me";
        assert_eq!(vec!["Rust:", "Trust me"], search(query, contents, true));
    }
}
