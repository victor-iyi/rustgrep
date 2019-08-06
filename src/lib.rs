
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Too few arguments.");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // Read file contents.
  let haystack: String = fs::read_to_string(&config.filename)?;

  let found = search(&config.query, &haystack);
  for f in &found {
    println!("Found: {}", f);
  }

  Ok(())
}

pub fn search<'a>(needle: &'a str, haystack: &'a str) -> Vec<&'a str> {
  let mut results: Vec<&str> = Vec::new();
  for line in haystack.lines() {
    if line.contains(needle) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search_one() {
    let needle = "duct";
    let haystack = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(needle, haystack));
  }

  #[test]
  fn search_two() {
    let needle = "st";
    let haystack = "\
Rust:
safe, fast, productive.
Pick three.
    ";
    assert_eq!(
      vec!["Rust:", "safe, fast, productive."],
      search(needle, haystack)
    );
  }
}