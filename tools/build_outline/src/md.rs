use regex::Regex;

/// get headings in markdown text
pub fn select_headings(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#{1,6} .+").unwrap();
    }

    return RE
        .find_iter(text)
        .map(|m| format!("{}\n\n", m.as_str()))
        .collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::select_headings;

    #[test]
    fn it_should_return_headings_separated_by_newline() {
        let result = select_headings("# heading\nother text\n## heading2");

        assert_eq!(result.as_str(), "# heading\n## heading2")
    }

    #[test]
    fn it_should_return_empty_string_when_no_headings() {
        let result = select_headings("**other text**");

        assert_eq!(result.as_str(), "")
    }
}
