use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub insensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage : not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let insensitive = env::var("Case_Insensitive").is_err();
        Ok(Config { query, filename, insensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Fail");
    let result = if(config.insensitive) {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results

}


pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results

}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query="duct";
        let content="\
Rust
safe, fast, productive.
Pick Three
Duck tape";
        assert_eq!(
            vec!["safe, fast, productive."], search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query="rUst";
        let content="\
Rust
safe, fast, productive.
Pick Three
Trust me";
        assert_eq!(
            vec!["Rust", "Trust me"], search_case_insensitive(query, content)
        );
    }
}
