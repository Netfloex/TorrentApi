use crate::utils::normalize_title;
use distance::levenshtein;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

lazy_static! {
    static ref REMOVE_AFTER_YEAR: Regex = Regex::new(r"\W*(19|20)\d\d\D.*").unwrap();
    static ref REMOVE_TAGS_REGEX: Regex = Regex::new(r"\[.*\]|\(.*\)").unwrap();
    static ref YEAR_REGEX: Regex = Regex::new(r"(19|20)\d\d").unwrap();
    static ref SITE_REGEX: Regex = Regex::new(r"(www\.)?\w+\.(com|me|to)").unwrap();
    static ref BOUNDARIES_REGEX: Regex = Regex::new(r"[-._:]").unwrap();
}

pub fn parse_title(title: &str) -> String {
    let title = normalize_title(title);

    let Some(year) = YEAR_REGEX.find(&title) else {
        return String::new();
    };

    let title = REMOVE_AFTER_YEAR.replace(&title, "");
    let title = REMOVE_TAGS_REGEX.replace_all(&title, "");
    let title = SITE_REGEX.replace_all(&title, "");
    let title = BOUNDARIES_REGEX.replace_all(&title, " ");

    let title = title.trim();

    format!("{} ({})", title, year.as_str())
}

fn levenshtein_percentage(first: &str, second: &str) -> f64 {
    let distance = levenshtein(first, second);

    let max_len = first.len().max(second.len());

    1.0 - (distance as f64 / max_len as f64)
}

pub fn is_title_match(movie_title: &str, og_torrent_title: &str) -> bool {
    let torrent_title = parse_title(og_torrent_title);

    let movie_title = normalize_title(movie_title);

    let match_perc = levenshtein_percentage(&movie_title, &torrent_title);

    let matches = match_perc > 0.8;
    if !matches {
        debug!("Incorrect movie: {}", torrent_title)
    }
    matches
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::r#static::tests::matrix_torrents::TestMatrixTorrents;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_MATRIX_TORRENTS: TestMatrixTorrents = TestMatrixTorrents::new();
    }

    #[test]
    fn test_parse_title() {
        TEST_MATRIX_TORRENTS.get().iter().for_each(|torrent| {
            assert_eq!(
                &parse_title(torrent.name()),
                torrent.parsed(),
                "{}",
                torrent.name()
            );
        });
    }

    #[test]
    fn test_levenshtein_percentage_1() {
        let same = "aaaa";
        assert_eq!(levenshtein_percentage(same, same), 1.0);
    }

    #[test]
    fn test_levenshtein_percentage_05() {
        let left_ = "aaaa";
        let right = "aabb";
        assert_eq!(levenshtein_percentage(left_, right), 0.5);
    }

    #[test]
    fn test_levenshtein_percentage_0() {
        let left_ = "aaaa";
        let right = "bbbb";
        assert_eq!(levenshtein_percentage(left_, right), 0.0);
    }

    #[test]
    fn test_boundaries_regex() {
        let boundaries = "_.:-";
        assert_eq!(BOUNDARIES_REGEX.replace_all(boundaries, ""), "");
    }

    #[test]
    fn test_remove_tags_regex() {
        let tags = "[test]middle(test)";
        assert_eq!(REMOVE_TAGS_REGEX.replace_all(tags, ""), "middle");
    }

    #[test]
    fn test_year_regex() {
        let year = "test test.1999 test";
        assert_eq!(YEAR_REGEX.find(year).unwrap().as_str(), "1999");
    }

    #[test]
    fn test_site_regex() {
        let site = "start www.test.com end";
        assert_eq!(SITE_REGEX.replace_all(site, ""), "start  end");
    }

    #[test]
    fn test_remove_after_year_regex() {
        let remove_after_year = "1999 test";
        assert_eq!(REMOVE_AFTER_YEAR.replace(remove_after_year, ""), "");
    }
}
