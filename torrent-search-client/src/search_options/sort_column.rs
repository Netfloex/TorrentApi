use std::str::FromStr;

use super::invalid_option_error::{InvalidOptionError, SearchOption};

#[derive(Debug, Default, Clone)]
pub enum SortColumn {
    Added,
    Size,
    Leechers,
    #[default]
    Seeders,
}

impl FromStr for SortColumn {
    type Err = InvalidOptionError;

    fn from_str(s: &str) -> Result<Self, InvalidOptionError> {
        let sort_column = match s.to_ascii_lowercase().as_str() {
            "time" => SortColumn::Added,
            "date" => SortColumn::Added,
            "added" => SortColumn::Added,

            "size" => SortColumn::Size,

            "leechers" => SortColumn::Leechers,
            "seeders" => SortColumn::Seeders,

            _ => Err(InvalidOptionError::new(SearchOption::Sort))?,
        };

        Ok(sort_column)
    }
}
