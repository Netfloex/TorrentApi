mod category;
mod order;
mod sort_column;

pub use self::category::Category;
pub use self::{order::Order, sort_column::SortColumn};

pub struct SearchOptions {
    query: String,
    category: Category,
    sort: SortColumn,
    order: Order,
}

impl SearchOptions {
    pub fn query(&self) -> &String {
        &self.query
    }
    pub fn category(&self) -> &Category {
        &self.category
    }
    pub fn sort(&self) -> &SortColumn {
        &self.sort
    }
    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn new(query: String, category: Category, sort: SortColumn, order: Order) -> Self {
        Self {
            query,
            category,
            sort: sort,
            order: order,
        }
    }
}
