use std::str::FromStr;

use crate::error::{Error, ErrorKind};

#[derive(Debug)]
pub enum SortColumn {
    Added(),
    Size(),
    Leechers(),
    Seeders(),
}

impl FromStr for SortColumn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let sort_column = match s.to_ascii_lowercase().as_str() {
            "time" => SortColumn::Added(),
            "date" => SortColumn::Added(),
            "added" => SortColumn::Added(),

            "size" => SortColumn::Size(),

            "leechers" => SortColumn::Leechers(),
            "seeders" => SortColumn::Seeders(),

            _ => Err(Error::new(ErrorKind::InvalidString(), "Incorrect column"))?,
        };

        Ok(sort_column)
    }
}
