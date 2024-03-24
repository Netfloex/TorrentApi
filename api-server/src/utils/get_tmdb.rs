use lazy_static::lazy_static;
use movie_info::TmdbId;
use regex::Regex;
lazy_static! {
    static ref IMDB_REGEX: Regex = Regex::new(r"\((\d{1,8})\)$").unwrap();
}
pub fn get_tmdb(name: &str) -> Option<TmdbId> {
    IMDB_REGEX
        .captures(name)
        .and_then(|c| c.get(1).map(|m| m.as_str().parse().ok()))
        .flatten()
}
