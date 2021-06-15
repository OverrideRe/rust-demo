
use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        Ok(Config { 
            query: query,
            file_name: file_name,
            case_sensitive: env::var("CASE_SENSITIVE").is_err() 
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}