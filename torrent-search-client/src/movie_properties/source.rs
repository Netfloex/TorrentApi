use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub enum Source {
    #[default]
    Unknown,
    BluRay,
    WebRip,
}

impl FromStr for Source {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codec = match s.to_ascii_lowercase().as_str() {
            "bluray" => Source::BluRay,
            "web" => Source::WebRip,
            _ => Self::Unknown,
        };

        Ok(codec)
    }
}
