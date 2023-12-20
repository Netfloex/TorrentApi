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
