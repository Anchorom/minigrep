use std::{env, error::Error, fs};
pub struct Config {
    pub target: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let target = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't get a target!"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't get a file path!"),
        };
        let ignore_case: bool = env::var("IGNORE_CASE").map_or_else(
            |_| args.any(|arg| arg.to_lowercase() == "-i" || arg.to_lowercase() == "--ignore_case"),
            |env_arg| env_arg == "1" || env_arg.to_lowercase() == "true",
        );

        Ok(Config {
            target,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_insensitive(&config.target, &contents)
    } else {
        search_sensitive(&config.target, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}

fn search_sensitive<'a>(target: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(target))
        .collect()
}

fn search_insensitive<'a>(target: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&target.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let target = "ast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_sensitive(target, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let target = "AsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_insensitive(target, contents)
        );
    }
}
