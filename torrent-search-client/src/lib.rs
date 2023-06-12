mod category;
mod client;
mod error;
mod http;
mod torrent;
use std::vec;

pub use category::Category;
use client::piratebay::PirateBay;
use client::x1337::X1137;
use client::TorrentProvider;
use error::Error;
use futures::future::join_all;
use http::create_http_client;
use reqwest_middleware::ClientWithMiddleware;
pub use torrent::Torrent;

pub struct TorrentClient {
    http: ClientWithMiddleware,
}

impl TorrentClient {
    pub async fn search_all<S: AsRef<str>>(
        &self,
        query: S,
        category: &Category,
    ) -> Vec<Result<Vec<Torrent>, Error>> {
        join_all(vec![
            X1137::search(query.as_ref(), category, &self.http),
            PirateBay::search(query.as_ref(), category, &self.http),
        ])
        .await
    }

    pub fn new() -> Self {
        let http = create_http_client();
        Self { http }
    }
}
