use regex::Regex;

/// return headers found in markdown text
pub fn select_headings(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#{1,6} .+").unwrap();
    }

    return RE
        .find_iter(text)
        .map(|m| format!("{} \n", m.as_str()))
        .collect::<String>();
}
