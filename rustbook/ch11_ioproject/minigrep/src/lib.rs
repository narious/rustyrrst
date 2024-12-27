use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return  Err("not enough args");
        }
        // Small performance hit when using clone - could also use String reference but need to define lifetime.
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
        
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let contents = fs::read_to_string(config.file_path)?;
    Ok(())
}

// In this case the return should reference the lifetime of the contents parameter
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 1. Iterate through lines in content, 2. check if it contains query, 3. add that to a vector of str
    let mut result: Vec<&str> = vec![];
    let v: Vec<&str> = contents.split('\n').collect();
    for line in v {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
