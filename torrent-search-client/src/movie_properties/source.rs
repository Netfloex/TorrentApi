use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]

pub enum Source {
    #[default]
    Unknown,
    Cam,
    Telesync,
    Telecine,
    Dvd,
    Hdtv,
    Hdrip,
    #[cfg_attr(feature = "graphql", graphql(name = "WEBRIP"))]
    WebRip,
    #[cfg_attr(feature = "graphql", graphql(name = "WEBDL"))]
    WebDL,
    #[cfg_attr(feature = "graphql", graphql(name = "BLURAY"))]
    BluRay,
}

lazy_static! {
    static ref CAM_REGEX: Regex = Regex::new(r"\b(?:cam|hqcam|hdcam|camrip|hdcamrip)\b").unwrap();
    static ref TELESYNC_REGEX: Regex =
        Regex::new(r"\b(?:telesync|hd-?ts|ts|pdvd|predvdrip)\b").unwrap();
    static ref TELECINE_REGEX: Regex = Regex::new(r"\b(?:telecine|hd-?tc|tc)\b").unwrap();
    static ref DVD_REGEX: Regex = Regex::new(r"\b(?:dvd|dvdrip|xvidvd|dvdr)\b").unwrap();
    static ref HDTV_REGEX: Regex =
        Regex::new(r"\b(?:hdtv|pdtv|dsr|dsrrip|satrip|dthrip|dvbrip|dtvrip|tvrip|hdtvrip)\b")
            .unwrap();
    static ref HDRIP_REGEX: Regex = Regex::new(r"\b(?:hdrip|web-?dlrip)\b").unwrap();
    static ref WEBRIP_REGEX: Regex = Regex::new(r"\b(?:web-?rip)\b").unwrap();
    static ref WEBDL_REGEX: Regex = Regex::new(r"\b(?:web|web-?dl|webrip)\b").unwrap();
    static ref BLURAY_REGEX: Regex =
        Regex::new(r"\b(?:blu-?ray|bdrip|brip|brrip|bdr|bd|bdiso|bdmv|bdremux)\b").unwrap();
}

impl FromStr for Source {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = match s.to_ascii_lowercase().as_str() {
            s if CAM_REGEX.is_match(s) => Source::Cam,
            s if TELESYNC_REGEX.is_match(s) => Source::Telesync,
            s if TELECINE_REGEX.is_match(s) => Source::Telecine,
            s if DVD_REGEX.is_match(s) => Source::Dvd,
            s if HDTV_REGEX.is_match(s) => Source::Hdtv,
            s if HDRIP_REGEX.is_match(s) => Source::Hdrip,
            s if WEBRIP_REGEX.is_match(s) => Source::WebRip,
            s if WEBDL_REGEX.is_match(s) => Source::WebDL,
            s if BLURAY_REGEX.is_match(s) => Source::BluRay,

            _ => Self::Unknown,
        };

        Ok(source)
    }
}
