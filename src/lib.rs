use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    results.iter().for_each(|line| println!("{}", line));

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: minigrep <query> <file_path>");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            file_path: filename,
            case_sensitive: env::var("CASE_SENSITIVE").is_err(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive_search() {
        let query = "duct";
        let contents = "\
Duct:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Duct:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Duct:", "safe, fast, productive."], search_case_insensitive(query, contents));
    }
}