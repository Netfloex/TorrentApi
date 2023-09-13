use derive_getters::Getters;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub enum Quality {
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Default)]
pub enum VideoCodec {
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Default)]
pub enum Source {
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Getters)]
pub struct MovieProperties {
    quality: Quality,
    codec: VideoCodec,
    source: Source,
    imdb: String,
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
        if self.imdb().is_empty() {
            self.imdb = other.imdb
        }
    }

    pub fn new(imdb: String) -> Self {
        Self {
            quality: Quality::default(),
            codec: VideoCodec::default(),
            source: Source::default(),
            imdb,
        }
    }
}
