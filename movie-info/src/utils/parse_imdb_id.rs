use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref IMDB_ID_REGEX: Regex = Regex::new(r"(?i)tt\d{7,8}").unwrap();
}

pub fn parse_imdb_id(query: &str) -> Option<&str> {
    IMDB_ID_REGEX.find(query).map(|m| m.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_imdb_id_works() {
        assert_eq!(parse_imdb_id("tt1234567"), Some("tt1234567"));
        assert_eq!(parse_imdb_id("TT1234567"), Some("TT1234567"));

        assert_eq!(parse_imdb_id("aaa tt1234567 aaa"), Some("tt1234567"));
    }

    #[test]
    fn parse_imdb_id_fails() {
        assert_eq!(parse_imdb_id("tt123456"), None);
        assert_eq!(parse_imdb_id("aa12345678"), None);
        assert_eq!(parse_imdb_id("1234567"), None);
    }
}
