use regex::Regex;

/// Narrow a string to just headings split by a blank line.
pub fn select_headings(text: String) {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"#{1,6} .+").unwrap();
  }

  RE.find_iter(&text.as_str()).map(|m| m.as_str());
}