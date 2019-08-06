
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

  let found: Vec<&str> = search(&config.query, &haystack);
  for (i, f) in found.iter().enumerate() {
    println!("Found {} - {}", i + 1, f);
  }

  Ok(())
}

pub fn search<'a>(needle: &'a str, haystack: &'a str) -> Vec<&'a str> {
  // An iterator over each lines in haystack,
  // filter those whose line contains needle,
  // and collect them into a vector of str.
  haystack
    .lines()
    .filter(|line| line.contains(needle))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search_zero() {
    let needle = "foo";
    let haystack = "\
Rust:
safe, fast, productive.
Pick three.";
    let empty: Vec<&str> = Vec::new();
    assert_eq!(empty, search(needle, haystack));
  }

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