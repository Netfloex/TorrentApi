// #[cfg(test)]
use crate::{Quality, Source, VideoCodec};
use derive_getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct TestMatrixTorrent {
    name: String,
    codec: VideoCodec,
    source: Source,
    quality: Quality,
    parsed: String,
}

pub struct TestMatrixTorrents {
    torrents: Vec<TestMatrixTorrent>,
}

impl TestMatrixTorrents {
    fn init() -> Vec<TestMatrixTorrent> {
        serde_json::from_str(include_str!("matrix_torrents.json")).unwrap()
    }

    pub fn new() -> Self {
        Self {
            torrents: Self::init(),
        }
    }

    pub fn get(&self) -> &Vec<TestMatrixTorrent> {
        &self.torrents
    }
}
