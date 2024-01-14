use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
    static ref IMDB_REGEX: Regex = Regex::new(r"\(([tT]{2}\d{8})\)").unwrap();
}
pub fn get_imdb(name: &str) -> Option<String> {
    IMDB_REGEX
        .captures(name)
        .map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .flatten()
}
