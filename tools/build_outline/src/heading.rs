use regex::Regex;

pub struct Heading<'a> {
  level: usize,
  text: &'a str,
}

impl Heading<'static> {
  pub fn new(level: usize, text: &str) -> Heading {
    Heading {
      level,
      text
    }
  }
}

pub fn extract_from_text(text: String) {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"#{1,6} .+").unwrap();
  }

  RE.captures_iter(&text.as_str()).map(|caps| -> Heading {
    Heading::new(
      caps.get(1).unwrap().as_str().len(), 
      caps.get(2).unwrap().as_str()
    )
  });
} 