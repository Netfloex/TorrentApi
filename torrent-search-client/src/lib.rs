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
use std::collections::HashSet;
use std::vec;
use surf::Client;
pub use torrent::Torrent;

#[derive(Default)]
pub struct TorrentClient {
    http: Client,
}

impl TorrentClient {
    pub async fn search_all(&self, search_options: &SearchOptions) -> Vec<ProviderResponse> {
        self.search(search_options, &Provider::all()).await
    }

    pub async fn search_movie_all(&self, movie_options: &MovieOptions) -> Vec<ProviderResponse> {
        self.search_movie(movie_options, &Provider::all()).await
    }

    pub async fn search(
        &self,
        search_options: &SearchOptions,
        providers: &HashSet<Provider>,
    ) -> Vec<ProviderResponse> {
        if search_options.query().is_empty() {
            return vec![];
        }

        let mut futures = vec![];

        let all_providers = Provider::all();

        let providers = if providers.is_empty() {
            &all_providers
        } else {
            providers
        };

        for provider in providers {
            match provider {
                Provider::X1337 => futures.push(X1137::search_provider(search_options, &self.http)),
                Provider::PirateBay => {
                    futures.push(PirateBay::search_provider(search_options, &self.http))
                }
                Provider::BitSearch => {
                    futures.push(BitSearch::search_provider(search_options, &self.http))
                }
                Provider::Yts => futures.push(Yts::search_provider(search_options, &self.http)),
            }
        }

        join_all(futures).await
    }

    pub async fn search_movie(
        &self,
        movie_options: &MovieOptions,
        providers: &HashSet<Provider>,
    ) -> Vec<ProviderResponse> {
        if movie_options.imdb().is_empty() {
            return vec![];
        }

        let mut futures = vec![];

        let all_providers = Provider::all();

        let providers = if providers.is_empty() {
            &all_providers
        } else {
            providers
        };

        for provider in providers {
            match provider {
                Provider::X1337 => {}
                Provider::PirateBay => {
                    futures.push(PirateBay::search_movies_provider(movie_options, &self.http))
                }
                Provider::BitSearch => {
                    futures.push(BitSearch::search_movies_provider(movie_options, &self.http))
                }
                Provider::Yts => {
                    futures.push(Yts::search_movies_provider(movie_options, &self.http))
                }
            }
        }

        join_all(futures).await
    }

    pub fn new() -> Self {
        Self {
            http: Client::new().with(SurfLogging),
        }
    }
}
