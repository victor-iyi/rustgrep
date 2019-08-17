pub fn search_sensitive<'a>(needle: &'a str, haystack: &'a str) -> Vec<&'a str> {
  // An iterator over each lines in haystack,
  // filter those whose line contains needle,
  // and collect them into a vector of str.
  haystack
    .lines()
    .filter(|line| line.contains(needle))
    .collect()
}

pub fn search_insensitive<'a>(needle: &'a str, haystack: &'a str) -> Vec<&'a str> {
  let needle: String = needle.to_lowercase();
  haystack
    .lines()
    .filter(|line| line.to_lowercase().contains(&needle))
    .collect()
}



macro_rules! search {
  ($needle:expr, $haystack:expr) => {{
    use crate::search;

    search::search_sensitive($needle, $haystack)
  }};

  ($needle:expr, $haystack:expr, $case:expr) => {{
    use crate::search;

    if $case {
      search::search_insensitive($needle, $haystack)
    } else {
      search::search_sensitive($needle, $haystack)
    }
  }};
}


#[cfg(test)]
mod tests {
  use super::*;

  // Haystack text to search for testing purposes.
  const HAYSTACK: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

  #[test]
  fn search_zero() {
    let empty: Vec<&str> = Vec::new();
    assert_eq!(empty, search_sensitive("foo", HAYSTACK));
  }

  #[test]
  fn search_one() {
    assert_eq!(vec!["I'm nobody! Who are you?"], search_sensitive("I'm", HAYSTACK));
  }

  #[test]
  fn search_two() {
    assert_eq!(
      vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
      search_sensitive("nobody", HAYSTACK)
    );
  }

  #[test]
  fn search_case_sensitive() {
    assert_eq!(vec!["Are you nobody, too?"], search_sensitive("Are", HAYSTACK));
  }

  #[test]
  fn search_case_insensitive() {
    assert_eq!(
      vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
      search_insensitive("are", HAYSTACK)
    );
  }
#[test]
  fn search_macro() {
    assert_eq!(vec!["Are you nobody, too?"], search!("Are", HAYSTACK));
  }

  #[test]
  fn search_macro_sensitive() {
    assert_eq!(
      vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
      search!("are", HAYSTACK, true)
    );
  }

  #[test]
  fn search_macro_insensitive() {
    assert_eq!(
      vec!["Are you nobody, too?"],
      search!("Are", HAYSTACK, false)
    );
  }
}
