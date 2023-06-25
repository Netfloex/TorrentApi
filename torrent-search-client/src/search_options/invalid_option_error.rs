use derive_getters::Getters;

#[derive(Debug)]

pub enum SearchOption {
    Category,
    Order,
    Sort,
}

impl ToString for SearchOption {
    fn to_string(&self) -> String {
        String::from(match self {
            SearchOption::Category => "category",
            SearchOption::Order => "order",
            SearchOption::Sort => "sort",
        })
    }
}

#[derive(Getters)]
pub struct InvalidOptionError {
    option: SearchOption,
}

impl InvalidOptionError {
    pub fn new(option: SearchOption) -> Self {
        Self { option }
    }
}
