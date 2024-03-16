use std::collections::HashSet;

use derive_getters::Getters;

#[derive(Debug, Getters, Default)]
pub struct Filters {
    imdb: bool,
    min_minutes: u64,
    languages: HashSet<String>,
}

impl Filters {
    pub fn new(imdb: bool, min_minutes: u64, languages: HashSet<String>) -> Self {
        Self {
            imdb,
            min_minutes,
            languages,
        }
    }
}
