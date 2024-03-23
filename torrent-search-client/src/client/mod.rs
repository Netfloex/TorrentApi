use std::collections::HashSet;

use crate::{
    error::Error,
    search_options::{movie_options::MovieOptions, SearchOptions},
    torrent::Torrent,
};
use async_trait::async_trait;
use serde::Serialize;
use strum::IntoEnumIterator;
use surf::Client;

pub mod bitsearch;
pub mod piratebay;
#[path = "1337x.rs"]
pub mod x1337;
pub mod yts;

pub struct ProviderResponse {
    pub provider: Provider,
    pub torrents: Result<Vec<Torrent>, Error>,
}

#[async_trait]
pub trait TorrentProvider {
    const PROVIDER: Provider;

    fn create_response(torrents: Result<Vec<Torrent>, Error>) -> ProviderResponse {
        ProviderResponse {
            provider: Self::PROVIDER,
            torrents,
        }
    }

    async fn search(search_options: &SearchOptions, http: &Client) -> Result<Vec<Torrent>, Error>;

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &Client,
    ) -> Result<Vec<Torrent>, Error>;

    async fn search_provider(search_options: &SearchOptions, http: &Client) -> ProviderResponse {
        let torrents = Self::search(search_options, http).await;

        Self::create_response(torrents)
    }

    async fn search_movies_provider(
        movie_options: &MovieOptions,
        http: &Client,
    ) -> ProviderResponse {
        let torrents = Self::search_movie(movie_options, http).await;

        Self::create_response(torrents)
    }
}

use strum_macros::EnumIter;
#[derive(EnumIter, Serialize, Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum Provider {
    #[cfg_attr(feature = "graphql", graphql(name = "PIRATEBAY"))]
    PirateBay,
    #[serde(rename = "1337x")]
    X1337,
    Yts,
    #[cfg_attr(feature = "graphql", graphql(name = "BITSEARCH"))]
    BitSearch,
}

impl Provider {
    pub fn all() -> HashSet<Provider> {
        Provider::iter().collect()
    }
}
