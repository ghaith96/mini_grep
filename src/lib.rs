use std::{env, error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
                query,
                filename,
                case_sensitive,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let mut contents = fs::read_to_string(config.filename)?;
    let mut query = config.query;

    if !config.case_sensitive {
        contents = contents.to_lowercase();
        query = query.to_lowercase();
    }

    search(&query, &contents)
        .iter()
        .for_each(|e| println!("{}", e));

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<_>>()
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        std::env::set_var("CASE_INSENSITIVE", "1");

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents));
    }
}
