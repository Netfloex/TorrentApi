use crate::{
    error::Error,
    search_options::{movie_options::MovieOptions, SearchOptions},
    torrent::Torrent,
};
use async_trait::async_trait;
use serde::Serialize;
use surf::Client;

pub mod bitsearch;
pub mod piratebay;
#[path = "1337x.rs"]
pub mod x1337;
pub mod yts;

#[async_trait]
pub trait TorrentProvider {
    async fn search(search_options: &SearchOptions, http: &Client) -> Result<Vec<Torrent>, Error>;

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &Client,
    ) -> Result<Vec<Torrent>, Error>;
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum Provider {
    #[cfg_attr(feature = "graphql", graphql(name = "PIRATEBAY"))]
    PirateBay,
    #[serde(rename = "1337x")]
    X1337,
    Yts,
    #[cfg_attr(feature = "graphql", graphql(name = "BITSEARCH"))]
    BitSearch,
}
