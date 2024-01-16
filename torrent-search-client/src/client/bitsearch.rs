use crate::utils::get_text;
use crate::{
    client::Provider,
    error::Error,
    movie_properties::MovieProperties,
    search_options::{MovieOptions, SearchOptions, SortColumn},
    torrent::Torrent,
    utils::{is_title_match, RoundRobin},
    Category, TorrentProvider,
};
use async_trait::async_trait;
use bytesize::ByteSize;
use chrono::{NaiveDateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::sync::Mutex;
use surf::{Client, Url};

const BITSEARCH_APIS: [&str; 2] = [
    "https://bitsearch.to/search",
    "https://solidtorrents.to/search",
];

lazy_static! {
    static ref ROW_SELECTOR: Selector = Selector::parse(".search-result").unwrap();
    static ref NAME_SELECTOR: Selector = Selector::parse("h5 a").unwrap();
    static ref MAGNET_SELECTOR: Selector = Selector::parse(".dl-magnet").unwrap();
    static ref CATEGORY_SELECTOR: Selector = Selector::parse(".category").unwrap();
    static ref STATS_SELECTOR: Selector = Selector::parse(".stats div").unwrap();
    static ref INFO_HASH_REGEX: Regex = Regex::new("urn:btih:([A-F\\d]+)").unwrap();
    static ref ROUND_ROBIN: Mutex<RoundRobin> = Mutex::new(RoundRobin::new(
        BITSEARCH_APIS
            .map(|url| url.parse::<Url>().unwrap())
            .to_vec()
    ));
}

pub struct BitSearch {}
impl BitSearch {
    fn get_url() -> Url {
        ROUND_ROBIN.lock().unwrap().get()
    }

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
        if number.contains('K') {
            return number.replace('K', "000").replace('.', "");
        }
        if number.contains('M') {
            return number.replace('M', "000000").replace('.', "");
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
        let mut url: Url = BitSearch::get_url();

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
    async fn search(search_options: &SearchOptions, http: &Client) -> Result<Vec<Torrent>, Error> {
        let url = BitSearch::format_url(search_options);

        let body = get_text::get_text(url, http).await?;

        let parsed = Html::parse_document(&body);

        let rows = parsed.select(&ROW_SELECTOR);

        fn get_text(item: Option<ElementRef>) -> String {
            item.map(|i| i.text().collect::<Vec<&str>>().join(""))
                .unwrap_or_default()
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

            let date = NaiveDateTime::parse_from_str(&format!("{date} 00:00"), "%b %d, %Y %R")
                .unwrap_or_default()
                .and_local_timezone(Utc)
                .unwrap();

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
            let name: String = row.select(&NAME_SELECTOR).next().unwrap().text().collect();

            torrents.push(Torrent {
                name: name.to_owned(),
                category: get_text(row.select(&CATEGORY_SELECTOR).next()),
                added: date,
                file_count: 0,
                id: info_hash.to_string(),
                info_hash,
                leechers: leechers.parse().unwrap(),
                seeders: seeders.parse().unwrap(),
                size: size.parse::<ByteSize>().unwrap().0,
                provider: Provider::BitSearch,
                magnet,
                movie_properties: Some(MovieProperties::new(
                    String::new(),
                    name.parse().expect("Should not return error"),
                    name.parse().expect("Should not return error"),
                    name.parse().expect("Should not return error"),
                )),
            })
        });

        Ok(torrents)
    }

    async fn search_movie(
        movie_options: &MovieOptions,
        http: &Client,
    ) -> Result<Vec<Torrent>, Error> {
        if let Some(title) = movie_options.title() {
            let options = SearchOptions::new(
                title.to_string(),
                Category::Video,
                movie_options.sort().clone(),
                movie_options.order().clone(),
            );

            let mut torrents = Self::search(&options, http).await?;

            torrents.retain(|t| is_title_match(title, t.name()));

            return Ok(torrents);
        }

        Ok(Vec::new())
    }
}
