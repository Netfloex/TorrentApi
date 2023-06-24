use std::str::FromStr;

use crate::error::{Error, ErrorKind};

#[derive(Default, Debug)]
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let order = match s.to_ascii_lowercase().as_str() {
            "all" => Category::All,
            "applications" => Category::Applications,
            "audio" => Category::Audio,
            "games" => Category::Games,
            "other" => Category::Other,
            "video" => Category::Video,
            _ => Err(Error::new(ErrorKind::InvalidString(), "Incorrect Category"))?,
        };

        Ok(order)
    }
}
