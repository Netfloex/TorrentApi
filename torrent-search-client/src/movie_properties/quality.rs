use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum Quality {
    #[default]
    Unknown,
    #[serde(rename = "480p")]
    P480,
    #[serde(rename = "540p")]
    P540,
    #[serde(rename = "576p")]
    P576,
    #[serde(rename = "720p")]
    P720,
    #[serde(rename = "1080p")]
    P1080,
    #[serde(rename = "2160p")]
    P2160,
}

lazy_static! {
    static ref P480_REGEX: Regex = Regex::new(r"\b(?:480p|640x480|848x480)\b").unwrap();
    static ref P540_REGEX: Regex = Regex::new(r"\b(?:540p)\b").unwrap();
    static ref P576_REGEX: Regex = Regex::new(r"\b(?:576p)\b").unwrap();
    static ref P720_REGEX: Regex =
        Regex::new(r"\b(?:720p|1280x720|960p|hdcam(?:rip)?|hdt[cs])\b").unwrap();
    static ref P1080_REGEX: Regex =
        Regex::new(r"\b(?:1080p|1920x1080|1440p|FHD|1080i|4kto1080p)\b").unwrap();
    static ref P2160_REGEX: Regex = Regex::new(
        r"\b(?:2160p|3840x2160|4k[-_. ](?:UHD|HEVC|BD|H\.?265)|(?:UHD|HEVC|BD|H\.?265)[-_. ]4k)\b"
    )
    .unwrap();
}

impl FromStr for Quality {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let quality = match s.to_ascii_lowercase().as_str() {
            s if P480_REGEX.is_match(s) => Quality::P480,
            s if P540_REGEX.is_match(s) => Quality::P540,
            s if P576_REGEX.is_match(s) => Quality::P576,
            s if P720_REGEX.is_match(s) => Quality::P720,
            s if P1080_REGEX.is_match(s) => Quality::P1080,
            s if P2160_REGEX.is_match(s) => Quality::P2160,
            _ => Self::Unknown,
        };

        Ok(quality)
    }
}
