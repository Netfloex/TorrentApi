use self::{codec::VideoCodec, quality::Quality, source::Source};
use derive_getters::Getters;
use serde::Serialize;

pub mod codec;
pub mod quality;
pub mod source;

#[derive(Debug, Clone, Serialize, Getters)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct MovieProperties {
    quality: Quality,
    codec: VideoCodec,
    source: Source,
    imdb: Option<String>,
}

impl MovieProperties {
    pub fn merge(&mut self, other: Self) {
        if matches!(self.codec, VideoCodec::Unknown) {
            self.codec = other.codec
        }
        if matches!(self.quality, Quality::Unknown) {
            self.quality = other.quality
        }
        if matches!(self.source, Source::Unknown) {
            self.source = other.source
        }
        if self.imdb().is_none() {
            self.imdb = other.imdb
        }
    }

    pub fn new(imdb: String, quality: Quality, codec: VideoCodec, source: Source) -> Self {
        Self {
            quality,
            codec,
            source,
            imdb: if imdb.is_empty() { None } else { Some(imdb) },
        }
    }
}
