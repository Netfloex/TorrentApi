use super::Error;
use crate::{
    search_options::{category::Category, movie_options::MovieOptions, SearchOptions},
    torrent::Torrent,
    utils::get_json::get_json,
    Provider, TorrentProvider,
};
use async_trait::async_trait;
use getset::Getters;
use lazy_static::lazy_static;
use serde::Deserialize;
use surf::{Client, Url};

#[derive(Deserialize, Debug, Getters)]
#[get = "pub"]
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

    async fn search_request(url: Url, http: &Client) -> Result<Vec<Torrent>, Error> {
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
    const PROVIDER: Provider = Provider::PirateBay;

    async fn search(search_options: &SearchOptions, http: &Client) -> Result<Vec<Torrent>, Error> {
        let url = PirateBay::format_url(search_options);

        let torrents = PirateBay::search_request(url, http).await?;

        Ok(torrents)
    }

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &Client,
    ) -> Result<Vec<Torrent>, Error> {
        let url = PirateBay::format_movie_url(movie_options);

        let mut torrents = PirateBay::search_request(url, http).await?;

        torrents.retain(|torrent| {
            if let Some(torrent_properties) = &torrent.movie_properties {
                if let Some(imdb) = torrent_properties.get_imdb() {
                    imdb == movie_options.imdb()
                } else {
                    false
                }
            } else {
                false
            }
        });

        Ok(torrents)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Order, SortColumn};

    use super::*;

    #[test]
    fn test_format_url() {
        let search_options = SearchOptions::new(
            "query".into(),
            Category::Applications,
            SortColumn::Seeders,
            Order::Ascending,
        );

        let url = PirateBay::format_url(&search_options);
        assert_eq!(url.as_str(), "https://apibay.org/q.php?q=query&cat=300");
    }

    #[test]
    fn test_format_movie_url() {
        let movie_options = MovieOptions::new(
            "tt1234567".into(),
            None,
            SortColumn::Seeders,
            Order::Ascending,
        );

        let url = PirateBay::format_movie_url(&movie_options);
        assert_eq!(url.as_str(), "https://apibay.org/q.php?q=tt1234567");
    }

    #[test]
    fn test_format_category() {
        assert_eq!(PirateBay::format_category(&Category::All), "");
        assert_eq!(PirateBay::format_category(&Category::Applications), "300");
        assert_eq!(PirateBay::format_category(&Category::Audio), "100");
        assert_eq!(PirateBay::format_category(&Category::Video), "200");
        assert_eq!(PirateBay::format_category(&Category::Games), "400");
        assert_eq!(PirateBay::format_category(&Category::Other), "600");
    }
}
