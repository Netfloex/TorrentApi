mod order;
mod sort_column;

use crate::Category;

pub use self::{order::Order, sort_column::SortColumn};

pub struct SearchOptions {
    query: String,
    category: Category,
    sort: Option<SortColumn>,
    order: Option<Order>,
}

impl SearchOptions {
    pub fn query(&self) -> &String {
        &self.query
    }
    pub fn category(&self) -> &Category {
        &self.category
    }
    pub fn sort(&self) -> &Option<SortColumn> {
        &self.sort
    }
    pub fn order(&self) -> &Option<Order> {
        &self.order
    }

    pub fn new(
        query: String,
        category: Category,
        sort: Option<SortColumn>,
        order: Option<Order>,
    ) -> Self {
        Self {
            query,
            category,
            sort,
            order,
        }
    }
}
