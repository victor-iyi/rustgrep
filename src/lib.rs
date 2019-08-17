use std::error::Error;
use std::fs;

#[macro_use]
mod search;


#[derive(Debug)]
pub struct Config {
  pub query: String,
  pub filename: String,
  pub case: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Too few arguments.");
    }

    let query = args[1].to_string();
    let filename = args[2].to_string();

    if args.len() == 4 {
      return Ok(Config {
        query,
        filename,
        case: args.contains(&"-c".to_string()),
      });
    }

    Ok(Config {
      query,
      filename,
      case: false,
    })
  }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // Read file contents.
  let haystack: String = fs::read_to_string(&config.filename)?;
  println!("Case: {}", config.case);
  println!("Needle: {}", config.query);
  println!("Haystack file: {}", config.filename);

  let found = search!(&config.query, &haystack, config.case);
  for (i, f) in found.iter().enumerate() {
    println!("Found {} - {}", i + 1, f);
  }

  Ok(())
}
