use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use strum_macros::EnumIter;
#[derive(EnumIter, Debug, Clone, Serialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum Codec {
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

impl<S: Into<String>> From<S> for Codec {
    fn from(s: S) -> Self {
        match s.into().to_ascii_lowercase().as_str() {
            s if AVC_REGEX.is_match(s) => Codec::AVC,
            s if HEVC_REGEX.is_match(s) => Codec::HEVC,
            s if XVID_REGEX.is_match(s) => Codec::XVid,

            _ => Self::Unknown,
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
    fn test_codecs() {
        TEST_MATRIX_TORRENTS.get().iter().for_each(|torrent| {
            assert_eq!(
                &Codec::from(torrent.name()),
                torrent.codec(),
                "{}",
                torrent.name()
            );
        });
    }
}
