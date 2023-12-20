use distance::levenshtein;
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::normalize_title;

lazy_static! {
    static ref REMOVE_AFTER_YEAR: Regex = Regex::new(r"\W*(19|20)\d\d\D.*").unwrap();
    static ref REMOVE_TAGS_REGEX: Regex = Regex::new(r"\[.*\]|\(.*\)").unwrap();
    static ref YEAR_REGEX: Regex = Regex::new(r"19|20\d\d").unwrap();
    static ref SITE_REGEX: Regex = Regex::new(r"(www\.)?\w+\.(com|me|to)").unwrap();
    static ref BOUNDARIES_REGEX: Regex = Regex::new(r"[-._:]").unwrap();
}

pub fn parse_title(title: &str) -> String {
    let title = normalize_title(&title);

    let Some(year) = YEAR_REGEX.find(&title) else {
        return String::new();
    };

    let title = REMOVE_AFTER_YEAR.replace(&title, "");
    let title = REMOVE_TAGS_REGEX.replace(&title, "");
    let title = SITE_REGEX.replace(&title, "");

    let title = title.trim();

    format!("{} ({})", title, year.as_str())
}

fn levenshtein_percentage(first: &str, second: &str) -> f64 {
    let distance = levenshtein(&first, &second);

    let max_len = first.len().max(second.len());

    let match_perc = 1.0 - (distance as f64 / max_len as f64);

    match_perc
}

pub fn is_title_match(movie_title: &str, og_torrent_title: &str) -> bool {
    let torrent_title = parse_title(og_torrent_title);

    let movie_title = normalize_title(movie_title);

    let match_perc = levenshtein_percentage(&movie_title, &torrent_title);

    let matches = match_perc > 0.8;
    if !matches {
        println!("Incorrect movie: {}", torrent_title)
    }
    matches
}
