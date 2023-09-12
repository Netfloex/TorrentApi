use derive_getters::Getters;

use crate::{Order, SortColumn};

#[derive(Getters)]
pub struct MovieOptions {
    imdb: String,
    title: Option<String>,
    sort: SortColumn,
    order: Order,
}

impl MovieOptions {
    pub fn new(imdb: String, title: Option<String>, sort: SortColumn, order: Order) -> Self {
        Self {
            imdb,
            title,
            sort,
            order,
        }
    }
}
