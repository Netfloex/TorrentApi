use std::vec;

use crate::{
    search_options::{movie_options::MovieOptions, SearchOptions},
    torrent::Torrent,
    utils::get_json::get_json,
    Category, Provider, SortColumn, TorrentProvider,
};
use async_trait::async_trait;
use getset::Getters;
use lazy_static::lazy_static;
use serde::Deserialize;
use surf::{Client, Url};

use super::Error;

#[derive(Deserialize, Debug, Getters)]
#[get = "pub"]
pub struct YtsTorrentResponse {
    size_bytes: u64,
    peers: usize,
    seeds: usize,
    date_uploaded_unix: i64,
    quality: String,
    hash: String,
    video_codec: String,
    #[serde(rename = "type")]
    source: String,
}

#[derive(Deserialize, Debug)]
struct YtsMovie {
    title_long: String,
    imdb_code: String,
    torrents: Option<Vec<YtsTorrentResponse>>,
}

#[derive(Deserialize, Debug)]
struct YtsSearchData {
    movies: Option<Vec<YtsMovie>>,
}

#[derive(Deserialize, Debug)]
struct YtsSearchResponse {
    data: YtsSearchData,
}

#[derive(Deserialize, Debug, Getters)]
#[get = "pub"]
pub struct YtsTorrent {
    data: YtsTorrentResponse,
    title: String,
    imdb: String,
}

#[derive(Deserialize, Debug)]
struct YtsMovieSearchData {
    movie: YtsMovie,
}

#[derive(Deserialize, Debug)]
struct YtsMovieSearchResponse {
    data: YtsMovieSearchData,
}

const YTS_API: &str = "https://yts.mx/api/v2";
lazy_static! {
    static ref YTS_URL: Url = YTS_API.parse().unwrap();
}
pub struct Yts {}

impl Yts {
    fn format_sort(column: &SortColumn) -> &str {
        match column {
            SortColumn::Added => "date_added",
            SortColumn::Leechers => "peers",
            SortColumn::Size => "", // unsupported
            SortColumn::Seeders => "seeds",
        }
    }

    fn format_search_url(search_options: &SearchOptions) -> Url {
        let mut url = YTS_URL.clone();

        url.path_segments_mut().unwrap().push("list_movies.json");

        url.query_pairs_mut()
            .append_pair("query_term", search_options.query())
            .append_pair("sort_by", Self::format_sort(search_options.sort()))
            .append_pair("order_by", &search_options.order().to_string());

        url
    }

    fn format_movie_url(movie_options: &MovieOptions) -> Url {
        let mut url = YTS_URL.clone();

        url.path_segments_mut().unwrap().push("movie_details.json");

        url.query_pairs_mut()
            .append_pair("imdb_id", movie_options.imdb());

        url
    }

    fn movie_to_torrents(movie: YtsMovie) -> Vec<YtsTorrent> {
        if let Some(torrents) = movie.torrents {
            torrents
                .into_iter()
                .map(|torrent| YtsTorrent {
                    data: torrent,
                    title: movie.title_long.to_string(),
                    imdb: movie.imdb_code.to_string(),
                })
                .collect::<Vec<YtsTorrent>>()
        } else {
            vec![]
        }
    }
}

#[async_trait]
impl TorrentProvider for Yts {
    const PROVIDER: Provider = Provider::Yts;

    async fn search(search_options: &SearchOptions, http: &Client) -> Result<Vec<Torrent>, Error> {
        if !matches!(search_options.category(), Category::All | Category::Video) {
            return Ok(Vec::new());
        }

        let url = Yts::format_search_url(search_options);

        let json: YtsSearchResponse = get_json(url, http).await?;

        let torrents: Vec<Torrent> = json
            .data
            .movies
            .unwrap_or_default()
            .into_iter()
            .flat_map(|tor| Yts::movie_to_torrents(tor).into_iter().map(Torrent::from))
            .collect();

        Ok(torrents)
    }

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &Client,
    ) -> Result<Vec<Torrent>, Error> {
        let url = Yts::format_movie_url(movie_options);
        let json: YtsMovieSearchResponse = get_json(url, http).await?;

        let yts_torrents = Yts::movie_to_torrents(json.data.movie);

        let torrents: Vec<Torrent> = yts_torrents.into_iter().map(Torrent::from).collect();

        Ok(torrents)
    }
}

#[cfg(test)]
mod tests {
    use crate::Order;

    use super::*;

    #[test]
    fn test_format_sort() {
        assert_eq!(Yts::format_sort(&SortColumn::Added), "date_added");
        assert_eq!(Yts::format_sort(&SortColumn::Leechers), "peers");
        assert_eq!(Yts::format_sort(&SortColumn::Size), "");
        assert_eq!(Yts::format_sort(&SortColumn::Seeders), "seeds");
    }

    #[test]
    fn test_format_search_url() {
        let search_options = SearchOptions::new(
            "query".into(),
            Category::Applications,
            SortColumn::Seeders,
            Order::Ascending,
        );

        let url = Yts::format_search_url(&search_options);
        assert_eq!(
            url.as_str(),
            "https://yts.mx/api/v2/list_movies.json?query_term=query&sort_by=seeds&order_by=asc"
        );
    }

    #[test]
    fn test_format_movie_url() {
        let movie_options = MovieOptions::new(
            "tt1234567".into(),
            None,
            SortColumn::Seeders,
            Order::Ascending,
        );

        let url = Yts::format_movie_url(&movie_options);

        assert_eq!(
            url.as_str(),
            "https://yts.mx/api/v2/movie_details.json?imdb_id=tt1234567"
        );
    }
}
