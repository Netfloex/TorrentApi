mod client;
mod http;
mod torrent;

use client::piratebay::PirateBay;
use client::TorrentProvider;
use http::HttpClient;
pub use torrent::Torrent;

pub struct TorrentClient {
    http: HttpClient,
}

impl TorrentClient {
    pub async fn search<S: AsRef<str>>(&self, query: S) -> Vec<Torrent> {
        PirateBay::search(query.as_ref(), &self.http).await
    }

    pub fn new() -> Self {
        let http = HttpClient::new();
        Self { http }
    }
}
