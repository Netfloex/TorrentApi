mod category;
mod client;
mod http;
mod torrent;

use std::vec;

pub use category::Category;
use client::piratebay::PirateBay;
use client::x1337::X1137;
use client::TorrentProvider;
use futures::future::join_all;
use http::create_http_client;
use reqwest_middleware::ClientWithMiddleware;
pub use torrent::Torrent;

pub struct TorrentClient {
    http: ClientWithMiddleware,
}

impl TorrentClient {
    pub async fn search<S: AsRef<str>>(&self, query: S, category: &Category) -> Vec<Torrent> {
        let futures = join_all(vec![
            X1137::search(query.as_ref(), category, &self.http),
            PirateBay::search(query.as_ref(), category, &self.http),
        ])
        .await;

        futures.into_iter().flatten().collect()
    }

    pub fn new() -> Self {
        let http = create_http_client();
        Self { http }
    }
}
