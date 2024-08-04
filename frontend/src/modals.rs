use regex::Regex;

//Rust.doc (-) currentFormat
pub fn date_test(date: &str) -> bool {
    let date_regex = Regex::new(r"^\d{2}-\d{2}-\d{4}$").unwrap();
    date_regex.is_match(date)
}