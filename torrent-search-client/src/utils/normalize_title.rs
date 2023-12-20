use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NORMALIZE_BOUNDARIES: Regex = Regex::new(r"[-._:]").unwrap();
    static ref MULTIPLE_SPACES: Regex = Regex::new(r"\s{2,}").unwrap();
}

pub fn normalize_title(title: &str) -> String {
    let title = title.to_lowercase();
    let title = NORMALIZE_BOUNDARIES.replace_all(&title, " ");
    let title = MULTIPLE_SPACES.replace_all(&title, " ");
    let title = title.replace(|c: char| !c.is_ascii(), "");

    title.trim().to_string()
}
