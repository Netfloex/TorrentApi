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
    AVC,
    HEVC,
    XVid,
}

lazy_static! {
    static ref AVC_REGEX: Regex = Regex::new(r"\b([xh].?264|avc)\b").unwrap();
    static ref HEVC_REGEX: Regex = Regex::new(r"\b([xh].?265|hevc)\b").unwrap();
    static ref XVID_REGEX: Regex = Regex::new(r"\bx-?vid(?:hd)?\b").unwrap();
}

impl VideoCodec {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            s if AVC_REGEX.is_match(s) => VideoCodec::AVC,
            s if HEVC_REGEX.is_match(s) => VideoCodec::HEVC,
            s if XVID_REGEX.is_match(s) => VideoCodec::XVid,

            _ => Self::Unknown,
        }
    }
}
