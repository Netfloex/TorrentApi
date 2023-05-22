mod client;
mod http;
mod torrent;

use client::piratebay::PirateBay;
use client::TorrentProvider;
use http::create_http_client;
use reqwest_middleware::ClientWithMiddleware;
pub use torrent::Torrent;

pub struct TorrentClient {
    http: ClientWithMiddleware,
}

impl TorrentClient {
    pub async fn search<S: AsRef<str>>(&self, query: S) -> Vec<Torrent> {
        PirateBay::search(query.as_ref(), &self.http).await
    }

    pub fn new() -> Self {
        let http = create_http_client();
        Self { http }
    }
}
