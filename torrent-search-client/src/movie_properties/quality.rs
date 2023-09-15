use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub enum Quality {
    #[default]
    Unknown,
    #[serde(rename = "720p")]
    P720,
    #[serde(rename = "1080p")]
    P1080,
    #[serde(rename = "2160p")]
    P2160,
}

impl FromStr for Quality {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codec = match s.to_ascii_lowercase().as_str() {
            "720p" => Quality::P720,
            "2160p" => Quality::P2160,
            "1080p" => Quality::P1080,
            _ => Self::Unknown,
        };

        Ok(codec)
    }
}
