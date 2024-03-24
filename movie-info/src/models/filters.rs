use std::collections::HashSet;

use crate::MovieInfo;

#[derive(Debug, Default)]
pub struct Filters {
    imdb: bool,
    min_minutes: u16,
    languages: HashSet<String>,
}

impl Filters {
    pub fn new(imdb: bool, min_minutes: u16, languages: HashSet<String>) -> Self {
        Self {
            imdb,
            min_minutes,
            languages,
        }
    }

    pub fn filter(&self, movies: &mut Vec<MovieInfo>) {
        if self.imdb {
            movies.retain(|m| m.get_imdb_id().is_some())
        }

        if self.min_minutes > 0 {
            movies.retain(|m| m.get_runtime() >= &(self.min_minutes))
        }

        if !self.languages.is_empty() {
            movies.iter_mut().for_each(|m| {
                m.certifications_mut()
                    .retain(|c| self.languages.contains(c.get_country()))
            })
        }
    }
}
