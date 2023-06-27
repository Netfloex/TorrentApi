use crate::{error::Error, search_options::SearchOptions, torrent::Torrent};
use async_trait::async_trait;
use reqwest_middleware::ClientWithMiddleware;

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
