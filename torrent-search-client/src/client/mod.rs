use async_trait::async_trait;

use crate::{http::HttpClient, torrent::Torrent};

pub mod piratebay;

#[async_trait]
pub trait TorrentProvider {
    async fn search(query: &str, http: &HttpClient) -> Vec<Torrent>;
}
