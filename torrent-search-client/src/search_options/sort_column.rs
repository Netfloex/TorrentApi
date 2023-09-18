use super::invalid_option_error::{InvalidOptionError, SearchOption};
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum SortColumn {
    #[default]
    Seeders,
    Added,
    Size,
    Leechers,
}

impl FromStr for SortColumn {
    type Err = InvalidOptionError;

    fn from_str(s: &str) -> Result<Self, InvalidOptionError> {
        let sort_column = match s.to_ascii_lowercase().as_str() {
            "time" | "date" | "added" => SortColumn::Added,

            "size" => SortColumn::Size,

            "leechers" => SortColumn::Leechers,
            "seeders" => SortColumn::Seeders,

            _ => Err(InvalidOptionError::new(SearchOption::Sort))?,
        };

        Ok(sort_column)
    }
}
