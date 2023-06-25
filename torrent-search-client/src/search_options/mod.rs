mod category;
mod invalid_option_error;
mod order;
mod sort_column;

use derive_getters::Getters;

pub use self::category::Category;
pub use self::{order::Order, sort_column::SortColumn};
pub use invalid_option_error::InvalidOptionError;
pub use invalid_option_error::SearchOption;

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
            sort: sort,
            order: order,
        }
    }
}
