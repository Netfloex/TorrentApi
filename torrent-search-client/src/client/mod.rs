use async_trait::async_trait;

use crate::http::HttpClient;

pub mod piratebay;

#[async_trait]
pub trait TorrentProvider {
    async fn search(query: &str, http: &HttpClient);
}
