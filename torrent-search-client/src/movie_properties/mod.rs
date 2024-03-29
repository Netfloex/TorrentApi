use self::{codec::Codec, quality::Quality, source::Source};
use getset::Getters;
use serde::Serialize;

pub mod codec;
pub mod quality;
pub mod source;

#[derive(Debug, Clone, Serialize, PartialEq, Getters)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[getset(get = "pub with_prefix")]
pub struct MovieProperties {
    quality: Quality,
    codec: Codec,
    source: Source,
    imdb: Option<String>,
}

impl MovieProperties {
    pub fn merge(&mut self, other: Self) {
        if matches!(self.codec, Codec::Unknown) {
            self.codec = other.codec
        }
        if matches!(self.quality, Quality::Unknown) {
            self.quality = other.quality
        }
        if matches!(self.source, Source::Unknown) {
            self.source = other.source
        }
        if self.imdb.is_none() {
            self.imdb = other.imdb
        }
    }

    pub fn new(imdb: String, quality: Quality, codec: Codec, source: Source) -> Self {
        Self {
            quality,
            codec,
            source,
            imdb: if imdb.is_empty() { None } else { Some(imdb) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut props1 = MovieProperties::new(
            "1".to_string(),
            Quality::Unknown,
            Codec::AVC,
            Source::Unknown,
        );

        let props2 = MovieProperties::new(
            "2".to_string(),
            Quality::P1080,
            Codec::Unknown,
            Source::BluRay,
        );

        props1.merge(props2);

        assert_eq!(props1.quality, Quality::P1080);
        assert_eq!(props1.codec, Codec::AVC);
        assert_eq!(props1.source, Source::BluRay);
        assert_eq!(props1.imdb, Some("1".to_string()));
    }
}
