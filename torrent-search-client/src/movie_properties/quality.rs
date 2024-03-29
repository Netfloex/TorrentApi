use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use strum_macros::EnumIter;
#[derive(EnumIter, Debug, Clone, Serialize, Default, PartialEq, Copy, Eq)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
#[cfg_attr(test, derive(serde::Deserialize))]
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
    static ref P480_REGEX: Regex = Regex::new(r"\D(?:480p|640x480|848x480)\b").unwrap();
    static ref P540_REGEX: Regex = Regex::new(r"\D(?:540p)\b").unwrap();
    static ref P576_REGEX: Regex = Regex::new(r"\D(?:576p)\b").unwrap();
    static ref P720_REGEX: Regex =
        Regex::new(r"\D(?:720p|1280x720|960p|hdcam(?:rip)?|hdt[cs])\b").unwrap();
    static ref P1080_REGEX: Regex =
        Regex::new(r"\D(?:1080p|1920x1080|1440p|FHD|1080i|4kto1080p)\b").unwrap();
    static ref P2160_REGEX: Regex = Regex::new(
        r"\D(?:2160p|3840x2160|4k[-_. ](?:UHD|HEVC|BD|H\.?265)|(?:UHD|HEVC|BD|H\.?265)[-_. ]4k)\b"
    )
    .unwrap();
    static ref SOME_DIGITS_REGEX: Regex = Regex::new(r"\d{3,4}").unwrap();
}

impl<S: Into<String>> From<S> for Quality {
    fn from(s: S) -> Self {
        let s = s.into();
        match s.to_ascii_lowercase().as_str() {
            s if P480_REGEX.is_match(s) => Quality::P480,
            s if P540_REGEX.is_match(s) => Quality::P540,
            s if P576_REGEX.is_match(s) => Quality::P576,
            s if P720_REGEX.is_match(s) => Quality::P720,
            s if P1080_REGEX.is_match(s) => Quality::P1080,
            s if P2160_REGEX.is_match(s) => Quality::P2160,
            _ => {
                let new_string = SOME_DIGITS_REGEX
                    .replace_all(&s, |caps: &regex::Captures| {
                        caps.get(0).unwrap().as_str().to_string() + "p"
                    })
                    .to_string();

                match new_string.as_str() {
                    s if P480_REGEX.is_match(s) => Quality::P480,
                    s if P540_REGEX.is_match(s) => Quality::P540,
                    s if P576_REGEX.is_match(s) => Quality::P576,
                    s if P720_REGEX.is_match(s) => Quality::P720,
                    s if P1080_REGEX.is_match(s) => Quality::P1080,
                    s if P2160_REGEX.is_match(s) => Quality::P2160,
                    _ => Self::Unknown,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#static::tests::matrix_torrents::TestMatrixTorrents;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_MATRIX_TORRENTS: TestMatrixTorrents = TestMatrixTorrents::new();
    }

    #[test]
    fn test_qualities() {
        TEST_MATRIX_TORRENTS.get().iter().for_each(|torrent| {
            assert_eq!(
                &Quality::from(torrent.name()),
                torrent.quality(),
                "{}",
                torrent.name()
            );
        });
    }
}
