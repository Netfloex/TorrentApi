mod client;
mod error;
mod movie_properties;
mod search_options;
mod r#static;
mod torrent;
mod utils;

use ::utils::surf_logging::SurfLogging;
use client::bitsearch::BitSearch;
use client::piratebay::PirateBay;
use client::x1337::X1137;
use client::yts::Yts;
pub use client::Provider;
use client::TorrentProvider;
pub use error::Error;
pub use error::ErrorKind;
use futures::future::join_all;
use http_cache_surf::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
pub use movie_properties::MovieProperties;
pub use movie_properties::Quality;
pub use movie_properties::Source;
pub use movie_properties::VideoCodec;
pub use search_options::Category;
pub use search_options::InvalidOptionError;
pub use search_options::MovieOptions;
pub use search_options::Order;
pub use search_options::SearchOption;
pub use search_options::SearchOptions;
pub use search_options::SortColumn;
use std::vec;
use surf::Client;
pub use torrent::Torrent;

pub struct TorrentClient {
    http: Client,
}

impl TorrentClient {
    pub async fn search_all(
        &self,
        search_options: &SearchOptions,
    ) -> Vec<Result<Vec<Torrent>, Error>> {
        if search_options.query().is_empty() {
            return vec![];
        }

        join_all(vec![
            X1137::search(search_options, &self.http),
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
        if movie_options.imdb().is_empty() {
            return vec![];
        }
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
            http: Client::new()
                .with(Cache(HttpCache {
                    mode: CacheMode::ForceCache,
                    manager: CACacheManager::default(),
                    options: HttpCacheOptions::default(),
                }))
                .with(SurfLogging),
        }
    }
}
