use crate::{
    get_json::get_json, search_options::SearchOptions, torrent::Torrent, Category, SortColumn,
    TorrentProvider,
};
use async_trait::async_trait;
use derive_getters::Getters;
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
    kind: String,
}

#[derive(Deserialize, Debug)]
struct YtsMovie {
    title_long: String,
    imdb_code: String,
    torrents: Vec<YtsTorrentResponse>,
}

#[derive(Deserialize, Debug)]
struct YtsData {
    movies: Vec<YtsMovie>,
}

#[derive(Deserialize, Debug)]
struct YtsResponse {
    data: YtsData,
}

#[derive(Deserialize, Debug, Getters)]
pub struct YtsTorrent {
    data: YtsTorrentResponse,
    title: String,
    imdb: String,
}

const YTS_API: &str = "https://yts.mx/api/v2/list_movies.json";
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

    fn format_url(search_options: &SearchOptions) -> Url {
        let mut url: Url = YTS_API.parse().unwrap();

        url.query_pairs_mut()
            .append_pair("query_term", search_options.query())
            .append_pair("sort_by", Self::format_sort(search_options.sort()))
            .append_pair("order_by", &search_options.order().to_string());
        url
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

        let url = Yts::format_url(search_options);

        let json: YtsResponse = get_json(url, http).await?;

        let yts_torrents: Vec<YtsTorrent> = json
            .data
            .movies
            .into_iter()
            .flat_map(|movie| {
                movie
                    .torrents
                    .into_iter()
                    .map(|torrent| YtsTorrent {
                        data: torrent,
                        title: movie.title_long.to_string(),
                        imdb: movie.imdb_code.to_string(),
                    })
                    .collect::<Vec<YtsTorrent>>()
            })
            .collect();

        let torrents: Vec<Torrent> = yts_torrents.into_iter().map(Torrent::from).collect();

        Ok(torrents)
    }
}
