use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub enum Quality {
    #[default]
    Unknown,
    #[serde(rename = "480p")]
    P480,
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
        let quality = match s.to_ascii_lowercase().as_str() {
            s if s.contains("480p") => Quality::P480,
            s if s.contains("720p") => Quality::P720,
            s if s.contains("1080p") => Quality::P1080,
            s if s.contains("2160p") => Quality::P2160,
            s if s.contains("4k") => Quality::P2160,
            _ => Self::Unknown,
        };

        Ok(quality)
    }
}
