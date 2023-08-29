use derive_getters::Getters;

use crate::{Category, Order, SortColumn};

#[derive(Getters)]
pub struct MovieOptions {
    title: String,
    imdb: String,
    category: Category,
    sort: SortColumn,
    order: Order,
}

impl MovieOptions {
    pub fn new(
        title: String,
        imdb: String,
        category: Category,
        sort: SortColumn,
        order: Order,
    ) -> Self {
        Self {
            title,
            imdb,
            category,
            sort,
            order,
        }
    }
}
