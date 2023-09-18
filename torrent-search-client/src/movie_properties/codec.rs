use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum VideoCodec {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    X264,
    X265,
}

lazy_static! {
    static ref X264_REGEX: Regex = Regex::new(r"\b([xh].?264|avc)\b").unwrap();
    static ref X265_REGEX: Regex = Regex::new(r"\b([xh].?265|hevc)\b").unwrap();
}
impl FromStr for VideoCodec {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codec = match s.to_ascii_lowercase().as_str() {
            s if X264_REGEX.is_match(s) => VideoCodec::X264,
            s if X265_REGEX.is_match(s) => VideoCodec::X265,

            _ => Self::Unknown,
        };

        Ok(codec)
    }
}
