use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filecontent = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &filecontent) {
        println!("{}", line);
    }

    if search(&config.query, &filecontent).len() < 1 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "0ff5ec";
        let filecontent = "\
Hello 0ff!
0ff5ec is rusty;
lets see.";

        assert_eq!(vec!["0ff5ec is rusty;"], search(query, filecontent));
    }
}