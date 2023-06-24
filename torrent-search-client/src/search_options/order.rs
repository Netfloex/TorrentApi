use std::str::FromStr;

use crate::error::{Error, ErrorKind};

#[derive(Debug, Default)]
pub enum Order {
    #[default]
    Descending,
    Ascending,
}

impl FromStr for Order {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let order = match s.to_ascii_lowercase().as_str() {
            "asc" => Order::Ascending,
            "ascending" => Order::Ascending,

            "desc" => Order::Descending,
            "descending" => Order::Descending,

            _ => Err(Error::new(ErrorKind::InvalidString(), "Incorrect order"))?,
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
