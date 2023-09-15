mod client;
mod error;
mod get_json;
mod logging_middleware;
mod movie_properties;
mod search_options;
mod torrent;

use client::bitsearch::BitSearch;
use client::piratebay::PirateBay;
use client::x1337::X1137;
use client::yts::Yts;
use client::TorrentProvider;
pub use error::Error;
pub use error::ErrorKind;
use futures::future::join_all;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use logging_middleware::LoggingMiddleware;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_middleware::ClientWithMiddleware;
pub use search_options::Category;
pub use search_options::InvalidOptionError;
pub use search_options::MovieOptions;
pub use search_options::Order;
pub use search_options::SearchOption;
pub use search_options::SearchOptions;
pub use search_options::SortColumn;
use std::vec;
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
            // X1137::search(search_options, &self.http),
            PirateBay::search(search_options, &self.http),
            Yts::search(search_options, &self.http),
            BitSearch::search(search_options, &self.http),
        ])
        .await
    }

    pub async fn search_movie_all(
        &self,
        movie_options: &MovieOptions,
    ) -> Vec<Result<Vec<Torrent>, Error>> {
        join_all(vec![
            // X1137::search_movie(movie_options, &self.http),
            PirateBay::search_movie(movie_options, &self.http),
            BitSearch::search_movie(movie_options, &self.http),
            Yts::search_movie(movie_options, &self.http),
        ])
        .await
    }

    pub fn new() -> Self {
        Self {
            http: ClientBuilder::new(Client::new())
                .with(LoggingMiddleware)
                .with(Cache(HttpCache {
                    mode: CacheMode::ForceCache,
                    manager: CACacheManager::default(),
                    options: None,
                }))
                .build(),
        }
    }
}
