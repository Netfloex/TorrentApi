use super::Error;
use crate::{
    get_json::get_json,
    search_options::{Category, MovieOptions, SearchOptions},
    torrent::Torrent,
    TorrentProvider,
};
use async_trait::async_trait;
use derive_getters::Getters;
use lazy_static::lazy_static;
use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

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
lazy_static! {
    static ref PIRATE_BAY_URL: Url = PIRATE_BAY_API.parse().unwrap();
}

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
        let mut url = PIRATE_BAY_URL.clone();

        url.query_pairs_mut()
            .append_pair("q", search_options.query())
            .append_pair("cat", Self::format_category(search_options.category()));

        url
    }

    fn format_movie_url(movie_options: &MovieOptions) -> Url {
        let mut url = PIRATE_BAY_URL.clone();

        url.query_pairs_mut().append_pair("q", movie_options.imdb());

        url
    }

    fn is_empty_torrent(torrent: &PirateBayTorrent) -> bool {
        torrent.id == "0"
            && torrent.size == "0"
            && torrent.category == "0"
            && torrent.num_files == "0"
            && torrent.added == "0"
    }

    async fn search_request(url: Url, http: &ClientWithMiddleware) -> Result<Vec<Torrent>, Error> {
        let pb_torrents: Vec<PirateBayTorrent> = get_json(url, http).await?;

        if pb_torrents.len() == 1 && Self::is_empty_torrent(&pb_torrents[0]) {
            return Ok(Vec::new());
        }

        let torrents: Vec<Torrent> = pb_torrents.into_iter().map(Torrent::from).collect();

        Ok(torrents)
    }
}

#[async_trait]
impl TorrentProvider for PirateBay {
    async fn search(
        search_options: &SearchOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        let url = PirateBay::format_url(search_options);

        let torrents = PirateBay::search_request(url, http).await?;

        Ok(torrents)
    }

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        let url = PirateBay::format_movie_url(movie_options);

        let mut torrents = PirateBay::search_request(url, http).await?;

        torrents.retain(|torrent| {
            if let Some(torrent_properties) = &torrent.movie_properties {
                torrent_properties.imdb() == movie_options.imdb()
            } else {
                false
            }
        });

        Ok(torrents)
    }
}
