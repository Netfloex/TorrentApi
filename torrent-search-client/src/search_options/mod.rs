mod category;
mod invalid_option_error;
mod movie_options;
mod order;
mod sort_column;

use derive_getters::Getters;

pub use self::category::Category;
pub use self::{order::Order, sort_column::SortColumn};
pub use invalid_option_error::InvalidOptionError;
pub use invalid_option_error::SearchOption;
pub use movie_options::MovieOptions;

#[derive(Getters)]
pub struct SearchOptions {
    query: String,
    category: Category,
    sort: SortColumn,
    order: Order,
}

impl SearchOptions {
    pub fn new(query: String, category: Category, sort: SortColumn, order: Order) -> Self {
        Self {
            query,
            category,
            sort,
            order,
        }
    }
}
