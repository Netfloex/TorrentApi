use derive_getters::Getters;

#[derive(Debug, Getters, Default)]
pub struct Filters {
    imdb: bool,
    min_minutes: u64,
}

impl Filters {
    pub fn new(imdb: bool, min_minutes: u64) -> Self {
        Self { imdb, min_minutes }
    }
}
