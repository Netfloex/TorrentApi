use std::ops::AddAssign;

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::normalize_title;

lazy_static! {
    static ref REMOVE_AFTER_YEAR: Regex = Regex::new(r"[\s(.]*\d{4}.*").unwrap();
    static ref REMOVE_TAGS_REGEX: Regex = Regex::new(r"[\[(].*[\])]").unwrap();
    static ref YEAR_REGEX: Regex = Regex::new(r"19|20\d\d").unwrap();
}

pub fn parse_title(title: &str) -> String {
    let Some(year) = YEAR_REGEX.find(&title) else {
        return String::new();
    };

    let title = REMOVE_AFTER_YEAR.replace(&title, "");
    let title = REMOVE_TAGS_REGEX.replace(&title, "");
    let title = normalize_title(&title);
    let title = title.trim();

    format!("{} ({})", title, year.as_str())
}

fn match_count(first: &str, second: &str) -> f32 {
    let mut count = 0;
    let max_count = usize::min(first.len(), second.len());

    let first = first.to_ascii_lowercase();
    let mut first = first.chars();

    let second = second.to_ascii_lowercase();
    let mut second = second.chars();

    for _ in 0..max_count {
        if let Some(a) = first.next() {
            if let Some(b) = second.next() {
                if a.eq_ignore_ascii_case(&b) {
                    count.add_assign(1);
                }
            }
        }
    }

    (count as f32) / (max_count as f32)
}

pub fn is_title_match(movie_title: &str, torrent_title: &str) -> bool {
    let torrent_title = parse_title(torrent_title);

    let movie_title = normalize_title(movie_title);
    let match_perc = match_count(&torrent_title, &movie_title);

    let matches = match_perc > 0.8;
    if !matches {
        println!("Incorrect movie: {}", torrent_title)
    }
    matches
}
