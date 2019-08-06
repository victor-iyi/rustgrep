
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

  println!("Needle: {}", config.query);
  println!("Haystack: {}", haystack);
  Ok(())
}