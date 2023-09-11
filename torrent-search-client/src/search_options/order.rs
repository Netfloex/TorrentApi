use std::str::FromStr;

use super::invalid_option_error::{InvalidOptionError, SearchOption};

#[derive(Debug, Default, Clone)]
pub enum Order {
    #[default]
    Descending,
    Ascending,
}

impl FromStr for Order {
    type Err = InvalidOptionError;

    fn from_str(s: &str) -> Result<Self, InvalidOptionError> {
        let order = match s.to_ascii_lowercase().as_str() {
            "asc" => Order::Ascending,
            "ascending" => Order::Ascending,

            "desc" => Order::Descending,
            "descending" => Order::Descending,

            _ => Err(InvalidOptionError::new(SearchOption::Order))?,
        };

        Ok(order)
    }
}

impl ToString for Order {
    fn to_string(&self) -> String {
        match self {
            Order::Ascending => String::from("asc"),
            Order::Descending => String::from("desc"),
        }
    }
}
