use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MULTIPLE_SPACES: Regex = Regex::new(r"\s{2,}").unwrap();
}

pub fn normalize_title(title: &str) -> String {
    let title = title.replace(|c: char| !c.is_ascii(), "");
    let title = title.to_lowercase();
    let title = MULTIPLE_SPACES.replace_all(&title, " ");

    title.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_title() {
        assert_eq!(normalize_title(" test    TEST   test "), "test test test");
    }

    #[test]
    fn test_normalize_title_unicode() {
        assert_eq!(normalize_title("test👍🏻"), "test");
    }

    #[test]
    fn test_space_regex() {
        assert_eq!(MULTIPLE_SPACES.find("test  test").unwrap().as_str(), "  ");
    }
}
