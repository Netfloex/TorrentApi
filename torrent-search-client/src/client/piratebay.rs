use crate::{
    get_json::get_json,
    search_options::{Category, SearchOptions},
    torrent::Torrent,
    TorrentProvider,
};
use async_trait::async_trait;
use derive_getters::Getters;
use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

use super::Error;

#[derive(Deserialize, Debug, Getters)]
pub struct PirateBayTorrent {
    id: String,
    name: String,
    info_hash: String,
    leechers: String,
    seeders: String,
    num_files: String,
    size: String,
    username: String,
    added: String,
    status: String,
    category: String,
    imdb: String,
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

    fn format_url(search_options: &SearchOptions) -> Url {
        let mut url: Url = PIRATE_BAY_API.parse().unwrap();

        url.query_pairs_mut()
            .append_pair("q", search_options.query())
            .append_pair("cat", Self::format_category(search_options.category()));

        url
    }

    fn is_empty_torrent(torrent: &PirateBayTorrent) -> bool {
        torrent.id == "0"
            && torrent.size == "0"
            && torrent.category == "0"
            && torrent.num_files == "0"
            && torrent.added == "0"
    }
}

#[async_trait]
impl TorrentProvider for PirateBay {
    async fn search(
        search_options: &SearchOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        let url = PirateBay::format_url(search_options);

        let pb_torrents: Vec<PirateBayTorrent> = get_json(url, http).await?;

        if pb_torrents.len() == 1 && Self::is_empty_torrent(&pb_torrents[0]) {
            return Ok(Vec::new());
        }

        let torrents: Vec<Torrent> = pb_torrents.into_iter().map(Torrent::from).collect();

        Ok(torrents)
    }
}
