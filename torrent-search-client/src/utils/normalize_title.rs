use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SITE_REGEX: Regex = Regex::new(r"www\.\w+\.com").unwrap();
    static ref NORMALIZE_BOUNDARIES: Regex = Regex::new(r"[\.:-]").unwrap();
    static ref MULTIPLE_SPACES: Regex = Regex::new(r"\s{2,}").unwrap();
}

pub fn normalize_title(title: &str) -> String {
    let title = SITE_REGEX.replace_all(&title, "");
    let title = NORMALIZE_BOUNDARIES.replace_all(&title, " ");
    let title = MULTIPLE_SPACES.replace_all(&title, " ");
    let title = title.replace(|c: char| !c.is_ascii(), "");

    title.trim().to_string()
}
