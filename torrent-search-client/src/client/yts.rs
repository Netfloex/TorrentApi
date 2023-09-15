use std::vec;

use crate::{
    get_json::get_json,
    search_options::{MovieOptions, SearchOptions},
    torrent::Torrent,
    Category, SortColumn, TorrentProvider,
};
use async_trait::async_trait;
use derive_getters::Getters;
use lazy_static::lazy_static;
use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

use super::Error;

#[derive(Deserialize, Debug, Getters)]
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
    movies: Vec<YtsMovie>,
}

#[derive(Deserialize, Debug)]
struct YtsSearchResponse {
    data: YtsSearchData,
}

#[derive(Deserialize, Debug, Getters)]
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
    async fn search(
        search_options: &SearchOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        if !matches!(search_options.category(), Category::All | Category::Video) {
            return Ok(Vec::new());
        }

        let url = Yts::format_search_url(search_options);

        let json: YtsSearchResponse = get_json(url, http).await?;

        let yts_torrents: Vec<YtsTorrent> = json
            .data
            .movies
            .into_iter()
            .flat_map(Yts::movie_to_torrents)
            .collect();

        let torrents: Vec<Torrent> = yts_torrents.into_iter().map(Torrent::from).collect();

        Ok(torrents)
    }

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        let url = Yts::format_movie_url(movie_options);
        let json: YtsMovieSearchResponse = get_json(url, http).await?;

        let yts_torrents = Yts::movie_to_torrents(json.data.movie);

        let torrents: Vec<Torrent> = yts_torrents.into_iter().map(Torrent::from).collect();

        Ok(torrents)
    }
}
