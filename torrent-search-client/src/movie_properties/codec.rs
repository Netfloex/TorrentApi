use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum VideoCodec {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    X264,
    X265,
}

impl FromStr for VideoCodec {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codec = match s.to_ascii_lowercase().as_str() {
            s if s.contains("x264") => VideoCodec::X264,
            s if s.contains("h264") => VideoCodec::X264,
            s if s.contains("h.264") => VideoCodec::X264,
            s if s.contains("x.264") => VideoCodec::X264,

            s if s.contains("x265") => VideoCodec::X265,
            s if s.contains("h265") => VideoCodec::X265,
            s if s.contains("h.265") => VideoCodec::X265,
            s if s.contains("x.265") => VideoCodec::X265,

            _ => Self::Unknown,
        };

        Ok(codec)
    }
}
