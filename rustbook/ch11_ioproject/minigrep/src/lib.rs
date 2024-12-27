use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return  Err("not enough args");
        }
        // Small performance hit when using clone - could also use String reference but need to define lifetime.
        let query = args[1].clone();
        let file_path = args[2].clone();
        
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
    // 1. Iterate through lines in content, 2. check if it contains query, 3. add that to a vector of str
    let mut result = Vec::new();
    let v = contents.split('\n'); // Returns an iterator

    for line in v { // Alternative is for line in contents.lines()
        if ignore_case {    
            if line.to_lowercase().contains(query.to_lowercase().as_str()) {
                result.push(line);
            }
        } else {
            if line.contains(query) {
                result.push(line);
            }
        }
    }
    result
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
