use crate::{category::Category, torrent::Torrent, TorrentProvider};
use async_trait::async_trait;
use reqwest::{Method, Url};
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;
use serde_json::from_slice;

use super::Error;

#[derive(Deserialize, Debug)]
pub struct PirateBayTorrent {
    pub id: String,
    pub name: String,
    pub info_hash: String,
    pub leechers: String,
    pub seeders: String,
    pub num_files: String,
    pub size: String,
    pub username: String,
    pub added: String,
    pub status: String,
    pub category: String,
    pub imdb: String,
}

const PIRATE_BAY_API: &str = "https://apibay.org/q.php";
pub struct PirateBay {}

impl PirateBay {
    fn format_category(category: &Category) -> &'static str {
        match category {
            Category::All => "",
            Category::Applications => "300",
            Category::Audio => "100",
            Category::Video => "200",
            Category::Games => "400",
            Category::Other => "600",
        }
    }

    fn format_url(query: &str, category: &Category) -> Url {
        let mut base_url = Url::parse(PIRATE_BAY_API).unwrap();
        base_url
            .query_pairs_mut()
            .append_pair("q", query)
            .append_pair("cat", Self::format_category(category));

        base_url
    }
}

#[async_trait]
impl TorrentProvider for PirateBay {
    async fn search(
        query: &str,
        category: &Category,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        let url = PirateBay::format_url(query, category);
        println!("Request to: {}", url);
        let response = http.request(Method::GET, url).send().await?;

        let body = response.bytes().await?;

        let pb_torrents: Vec<PirateBayTorrent> = from_slice(&body)?;

        let torrents: Vec<Torrent> = pb_torrents.into_iter().map(Torrent::from).collect();

        Ok(torrents)
    }
}
