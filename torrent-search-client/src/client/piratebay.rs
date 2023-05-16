use crate::{http::HttpClient, torrent::Torrent, TorrentProvider};
use async_trait::async_trait;
use reqwest::{Method, Url};
use serde::Deserialize;
use serde_json::from_slice;

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
    fn format_url(query: &str) -> Url {
        let mut base_url = Url::parse(PIRATE_BAY_API).unwrap();
        base_url.query_pairs_mut().append_pair("q", query);

        base_url
    }
}

#[async_trait]
impl TorrentProvider for PirateBay {
    async fn search(query: &str, http: &HttpClient) -> Vec<Torrent> {
        let url = PirateBay::format_url(query);

        let response = http.request(Method::GET, url).send().await.unwrap();

        let body = response.bytes().await.unwrap();

        let pb_torrents: Vec<PirateBayTorrent> = from_slice(&body).unwrap();

        let torrents = pb_torrents.into_iter().map(Torrent::from).collect();

        torrents
    }
}
