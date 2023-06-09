use crate::{error::Error, search_options::SearchOptions, torrent::Torrent};
use async_trait::async_trait;
use reqwest_middleware::ClientWithMiddleware;
use serde::Serialize;

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
}

#[derive(Serialize, Debug)]
pub enum Provider {
    PirateBay,
    #[serde(rename = "1337x")]
    X1337,
    Yts,
}
