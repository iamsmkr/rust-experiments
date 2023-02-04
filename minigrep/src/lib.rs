use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("Not enough arguments"); }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }

    pub fn build_from_iter(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("Query string missing from the arg list")
        };

        let file_path = match args.next() {
            Some(v) => v,
            None => return Err("File path missing from the arg list")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    let Config { query, file_path: _, ignore_case } = &config;

    let search_results = if *ignore_case {
        search_case_insensitive(&query, &content)
    } else {
        search(&query, &content)
    };

    for line in search_results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.contains(query) {
            results.push(trimmed);
        }
    }
    results
}

fn search_using_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.trim().contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.to_lowercase().contains(query) {
            results.push(trimmed);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape.";
        assert_eq!(vec!["safe, fast, productive.", "Duct tape."], search_case_insensitive(query, contents));
    }
}
