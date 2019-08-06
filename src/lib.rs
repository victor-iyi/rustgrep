
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

  // Haystack text to search for testing purposes.
  const HAYSTACK: &'static str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

  #[test]
  fn search_zero() {
    let empty: Vec<&str> = Vec::new();
    assert_eq!(empty, search("foo", HAYSTACK));
  }

  #[test]
  fn search_one() {
    assert_eq!(vec!["I'm nobody! Who are you?"], search("I'm", HAYSTACK));
  }

  #[test]
  fn search_two() {
    assert_eq!(
      vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
      search("nobody", HAYSTACK)
    );
  }
}