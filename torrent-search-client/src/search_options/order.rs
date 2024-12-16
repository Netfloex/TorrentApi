use std::{fmt::Display, str::FromStr};

use super::invalid_option_error::{InvalidOptionError, SearchOption};

#[derive(PartialEq, Debug, Default, Clone, Copy, Eq)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum Order {
    #[default]
    Descending,
    Ascending,
}

impl FromStr for Order {
    type Err = InvalidOptionError;

    fn from_str(s: &str) -> Result<Self, InvalidOptionError> {
        let order = match s.to_ascii_lowercase().as_str() {
            "a" | "asc" | "ascending" => Order::Ascending,

            "d" | "desc" | "descending" => Order::Descending,

            _ => Err(InvalidOptionError::new(SearchOption::Order))?,
        };

        Ok(order)
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Order::Ascending => "asc",
                Order::Descending => "desc",
            }
        )
    }
}
