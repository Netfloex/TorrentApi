use std::str::FromStr;

use super::invalid_option_error::{InvalidOptionError, SearchOption};

#[derive(Default, Debug, Eq, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum Category {
    #[default]
    All,
    Audio,
    Video,
    Applications,
    Games,
    Other,
}

impl FromStr for Category {
    type Err = InvalidOptionError;

    fn from_str(s: &str) -> Result<Self, InvalidOptionError> {
        let order = match s.to_ascii_lowercase().as_str() {
            "all" => Category::All,
            "applications" => Category::Applications,
            "audio" => Category::Audio,
            "games" => Category::Games,
            "other" => Category::Other,
            "video" => Category::Video,

            _ => Err(InvalidOptionError::new(SearchOption::Category))?,
        };

        Ok(order)
    }
}
