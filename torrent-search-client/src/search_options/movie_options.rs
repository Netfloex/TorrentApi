use derive_getters::Getters;

use crate::{Order, SortColumn};

#[derive(Getters)]
pub struct MovieOptions {
    imdb: String,
    sort: SortColumn,
    order: Order,
}

impl MovieOptions {
    pub fn new(imdb: String, sort: SortColumn, order: Order) -> Self {
        Self { imdb, sort, order }
    }
}
