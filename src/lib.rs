use std::error::Error;
use std::fs;
use std::env;

/// Searches for `query` in `contents` and returns a vector of lines
/// that match, using case insensitive search if `config.ignore_case` is
/// true. Prints each matching line and returns Ok(()) on success.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

/// Searches for `query` in `contents` case-insensitively and returns a
/// vector of lines that contain `query`. Converts `query` and each line
/// to lowercase before searching.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

/// Searches the given contents string for lines containing
/// the given query string. Returns a vector of references
/// to the matched lines. Performs a case-sensitive search.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
/// Builds a Config from the given command line arguments.
///
/// Parses the given arguments slice and returns a Config struct.
/// Requires at least 3 arguments: the program name, the query string,
/// and the file path. The ignore_case field is set based on the
/// IGNORE_CASE environment variable.
///
/// Returns a Result with the Config or a static error string if there are
/// not enough arguments.
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // The first argument should be the name of the package
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Didn't get a query string");
            }
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Didn't get a file path");
            }
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive;
Pick three.";

        assert_eq!(vec!["safe, fast, productive;"], search(query, contents));
    }

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
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
