use getset::Getters;
use std::fmt::Display;

#[derive(Debug)]

pub enum SearchOption {
    Category,
    Order,
    Sort,
}

impl Display for SearchOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SearchOption::Category => "category",
                SearchOption::Order => "order",
                SearchOption::Sort => "sort",
            }
        )
    }
}

#[derive(Getters)]
#[get = "pub"]
pub struct InvalidOptionError {
    option: SearchOption,
}

impl InvalidOptionError {
    pub fn new(option: SearchOption) -> Self {
        Self { option }
    }
}
