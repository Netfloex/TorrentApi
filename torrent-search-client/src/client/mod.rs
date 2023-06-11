use crate::{category::Category, torrent::Torrent};
use async_trait::async_trait;
use reqwest_middleware::ClientWithMiddleware;

pub mod piratebay;
#[path = "1337x.rs"]
pub mod x1337;

#[async_trait]
pub trait TorrentProvider {
    async fn search(query: &str, category: &Category, http: &ClientWithMiddleware) -> Vec<Torrent>;
}
