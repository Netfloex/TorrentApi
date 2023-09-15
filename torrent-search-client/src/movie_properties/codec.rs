use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum VideoCodec {
    #[default]
    Unknown,
    X264,
    X265,
}

impl FromStr for VideoCodec {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codec = match s.to_ascii_lowercase().as_str() {
            "x264" => VideoCodec::X264,
            "x265" => VideoCodec::X265,

            _ => Self::Unknown,
        };

        Ok(codec)
    }
}
