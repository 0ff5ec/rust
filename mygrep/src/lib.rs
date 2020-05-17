use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filecontent = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &filecontent)
    } else {
        search_case_insensitive(&config.query, &filecontent)
    };

    for line in &results {
        println!("{}", line);
    }

    if results.len() < 1 {
        println!("Query:{} not found", config.query);
    }

    Ok(())
}

pub fn search<'a>(query: &str, filecontent: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in filecontent.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    filecontent: &'a str
) -> Vec<&'a str> {

    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in filecontent.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "0ff5ec";
        let filecontent = "\
Hello 0ff!
0ff5ec is rusty;
lets see.
0ff5Ec";

        assert_eq!(vec!["0ff5ec is rusty;"], search(query, filecontent));
    }

    #[test]
    fn case_insensitive(){
        let query = "0ff5ec";
        let filecontent = "\
Hello 0ff!
0ff5ec is rusty;
lets see.
0ff5Ec";
        
        assert_eq!(vec!["0ff5ec is rusty;", "0ff5Ec"],
        search_case_insensitive(query, filecontent)
        );
    }
}