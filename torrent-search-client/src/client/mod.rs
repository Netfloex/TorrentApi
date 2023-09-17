use crate::{
    error::Error,
    search_options::{MovieOptions, SearchOptions},
    torrent::Torrent,
};
use async_trait::async_trait;
use juniper::GraphQLEnum;
use reqwest_middleware::ClientWithMiddleware;
use serde::Serialize;

pub mod bitsearch;
pub mod piratebay;
#[path = "1337x.rs"]
pub mod x1337;
pub mod yts;

#[async_trait]
pub trait TorrentProvider {
    async fn search(
        search_options: &SearchOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error>;

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error>;
}

#[derive(Serialize, Debug, Clone, GraphQLEnum)]
pub enum Provider {
    PirateBay,
    #[serde(rename = "1337x")]
    X1337,
    Yts,
    BitSearch,
}
