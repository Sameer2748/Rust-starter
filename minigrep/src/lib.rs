use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query , &contents)
    } else{
        search_case_sensitive(&config.query , &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive : bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("args not defined");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            res.push(line.trim());
        }
    }

    res
}
pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line.trim());
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "/
        Rust: 
        safe, fast, productive. 
        Pick three.
        Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_sensitive() {
        let query = "RusT";
        let content = "/
        Rust: 
        safe, fast, productive. 
        Pick three.
        Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_sensitive(query, content));
    }

 
}
