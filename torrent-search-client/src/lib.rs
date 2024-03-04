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
use client::ProviderResponse;
use client::TorrentProvider;
pub use error::Error;
pub use error::ErrorKind;
use futures::future::join_all;
use http_cache_surf::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
pub use movie_properties::codec::Codec;
pub use movie_properties::quality::Quality;
pub use movie_properties::source::Source;
pub use movie_properties::MovieProperties;
pub use search_options::category::Category;
pub use search_options::invalid_option_error::{InvalidOptionError, SearchOption};
pub use search_options::movie_options::MovieOptions;
pub use search_options::order::Order;
pub use search_options::sort_column::SortColumn;
pub use search_options::SearchOptions;
use std::vec;
use surf::Client;
pub use torrent::Torrent;

#[derive(Default)]
pub struct TorrentClient {
    http: Client,
}

impl TorrentClient {
    pub async fn search_all(&self, search_options: &SearchOptions) -> Vec<ProviderResponse> {
        if search_options.query().is_empty() {
            return vec![];
        }

        join_all(vec![
            X1137::search_provider(search_options, &self.http),
            PirateBay::search_provider(search_options, &self.http),
            BitSearch::search_provider(search_options, &self.http),
            Yts::search_provider(search_options, &self.http),
        ])
        .await
    }

    pub async fn search_movie_all(&self, movie_options: &MovieOptions) -> Vec<ProviderResponse> {
        if movie_options.imdb().is_empty() {
            return vec![];
        }

        join_all(vec![
            // X1137::search_movie_provider(movie_options, &self.http),
            PirateBay::search_movies_provider(movie_options, &self.http),
            BitSearch::search_movies_provider(movie_options, &self.http),
            Yts::search_movies_provider(movie_options, &self.http),
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
