mod category;
mod client;
mod error;
mod http;
mod search_options;
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
pub use search_options::Order;
pub use search_options::SearchOptions;
pub use search_options::SortColumn;
pub use torrent::Torrent;
pub struct TorrentClient {
    http: ClientWithMiddleware,
}

impl TorrentClient {
    pub async fn search_all(
        &self,
        search_options: &SearchOptions,
    ) -> Vec<Result<Vec<Torrent>, Error>> {
        join_all(vec![
            X1137::search(search_options, &self.http),
            PirateBay::search(search_options, &self.http),
        ])
        .await
    }

    pub fn new() -> Self {
        let http = create_http_client();
        Self { http }
    }
}
