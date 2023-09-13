use crate::{
    client::Provider,
    error::Error,
    search_options::{MovieOptions, SearchOptions, SortColumn},
    torrent::Torrent,
    Category, TorrentProvider,
};
use async_trait::async_trait;
use bytesize::ByteSize;
use chrono::{DateTime, NaiveDateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Method, Url};
use reqwest_middleware::ClientWithMiddleware;
use scraper::{ElementRef, Html, Selector};

const BITSEARCH_API: &str = "https://bitsearch.to/search";
lazy_static! {
    static ref BITSEARCH_URL: Url = BITSEARCH_API.parse().unwrap();
    static ref ROW_SELECTOR: Selector = Selector::parse(".search-result").unwrap();
    static ref NAME_SELECTOR: Selector = Selector::parse("h5 a").unwrap();
    static ref MAGNET_SELECTOR: Selector = Selector::parse(".dl-magnet").unwrap();
    static ref CATEGORY_SELECTOR: Selector = Selector::parse(".category").unwrap();
    static ref STATS_SELECTOR: Selector = Selector::parse(".stats div").unwrap();
    static ref INFO_HASH_REGEX: Regex = Regex::new("urn:btih:([A-F\\d]+)").unwrap();
}

pub struct BitSearch {}
impl BitSearch {
    fn format_category(category: &Category) -> &'static str {
        match category {
            Category::All => "",
            Category::Applications => "5",
            Category::Audio => "7",
            Category::Video => "1",
            Category::Games => "6",
            Category::Other => "",
        }
    }

    fn expand_number(number: &str) -> String {
        if number.contains("K") {
            return number.replace("K", "000").replace(".", "");
        }
        if number.contains("M") {
            return number.replace("M", "000000").replace(".", "");
        }

        number.to_owned()
    }

    fn format_sort(column: &SortColumn) -> &str {
        match column {
            SortColumn::Added => "date",
            SortColumn::Leechers => "leechers",
            SortColumn::Size => "size",
            SortColumn::Seeders => "seeders",
        }
    }

    fn format_url(search_options: &SearchOptions) -> Url {
        let mut url: Url = BITSEARCH_API.parse().unwrap();

        url.query_pairs_mut()
            .append_pair("q", search_options.query())
            .append_pair("sort", Self::format_sort(search_options.sort()))
            .append_pair("order", &search_options.order().to_string())
            .append_pair("category", Self::format_category(search_options.category()));

        url
    }
}

#[async_trait]
impl TorrentProvider for BitSearch {
    async fn search(
        search_options: &SearchOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        let url = BitSearch::format_url(search_options);

        let response = http
            .request(Method::GET, url)
            .send()
            .await?
            .error_for_status()?;

        let body = response.text().await?;

        let parsed = Html::parse_document(&body);

        let rows = parsed.select(&ROW_SELECTOR);

        fn get_text(item: Option<ElementRef>) -> String {
            item.and_then(|i| Some(i.text().collect::<Vec<&str>>().join("")))
                .unwrap_or(String::new())
                .trim()
                .to_string()
        }

        let mut torrents = vec![];

        rows.for_each(|row| {
            let mut stats = row.select(&STATS_SELECTOR);
            let size = get_text(stats.nth(1));
            let seeders = BitSearch::expand_number(&get_text(stats.next()));
            let leechers = BitSearch::expand_number(&get_text(stats.next()));
            let date = get_text(stats.next());

            let date = DateTime::<Utc>::from_utc(
                NaiveDateTime::parse_from_str(&format!("{date} 00:00"), "%b %d, %Y %R")
                    .unwrap_or_default(),
                Utc,
            );

            if size.is_empty() {
                return;
            }

            let magnet = row
                .select(&MAGNET_SELECTOR)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string()
                .replace("dn=%5BBitsearch.to%5D+", "dn=");

            let info_hash = INFO_HASH_REGEX
                .find(&magnet)
                .unwrap()
                .as_str()
                .replace("urn:btih:", "");

            torrents.push(Torrent {
                name: row.select(&NAME_SELECTOR).next().unwrap().text().collect(),
                category: get_text(row.select(&CATEGORY_SELECTOR).next()),
                added: date,
                file_count: 0,
                id: info_hash.to_string(),
                imdb: String::new(),
                info_hash,
                leechers: leechers.parse().unwrap(),
                seeders: seeders.parse().unwrap(),
                size: size.parse::<ByteSize>().unwrap().0,
                provider: Provider::BitSearch,
                magnet,
            })
        });

        Ok(torrents)
    }

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        if let Some(title) = movie_options.title() {
            let options = SearchOptions::new(
                title.to_string(),
                Category::Video,
                movie_options.sort().clone(),
                movie_options.order().clone(),
            );

            return Self::search(&options, http).await;
        }

        Ok(Vec::new())
    }
}
