use crate::utils::get_text;
use crate::{
    client::Provider,
    error::Error,
    movie_properties::MovieProperties,
    search_options::{movie_options::MovieOptions, sort_column::SortColumn, SearchOptions},
    torrent::Torrent,
    utils::{parse_title::is_title_match, round_robin::RoundRobin},
    Category, TorrentProvider,
};
use crate::{Codec, ErrorKind, Quality, Source};
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
            return number.replace('K', "00").replace('.', "");
        }
        if number.contains('M') {
            return number.replace('M', "00000").replace('.', "");
        }
        if number.contains('B') {
            return number.replace('B', "00000000").replace('.', "");
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
    const PROVIDER: Provider = Provider::BitSearch;

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

        for row in rows {
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
                continue;
            }

            // let a = ;
            let magnet = row
                .select(&MAGNET_SELECTOR)
                .next()
                .ok_or_else(|| {
                    Error::new(ErrorKind::ScrapingError, "Could not find the magnet link")
                })?
                .value()
                .attr("href")
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::ScrapingError,
                        "Magnet link does not have an href attribute",
                    )
                })?
                .to_string()
                .replace("dn=%5BBitsearch.to%5D+", "dn=");

            let info_hash = INFO_HASH_REGEX
                .find(&magnet)
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::ScrapingError,
                        "Could not find the info hash in the magnet link",
                    )
                })?
                .as_str()
                .replace("urn:btih:", "");

            let name: String = row
                .select(&NAME_SELECTOR)
                .next()
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::ScrapingError,
                        "Could not find the name in the html response",
                    )
                })?
                .text()
                .collect();

            torrents.push(Torrent {
                category: get_text(row.select(&CATEGORY_SELECTOR).next()),
                added: date,
                file_count: 0,
                id: info_hash.to_string(),
                info_hash,
                leechers: leechers.parse().map_err(|_| {
                    Error::new(ErrorKind::ScrapingError, "Leechers is not a number")
                })?,
                seeders: seeders
                    .parse()
                    .map_err(|_| Error::new(ErrorKind::ScrapingError, "Seeders is not a number"))?,
                size: size
                    .parse::<ByteSize>()
                    .map_err(|_| {
                        Error::new(
                            ErrorKind::ScrapingError,
                            "Size is not a number, or cannot be parsed by ByteSize",
                        )
                    })?
                    .0,
                provider: Provider::BitSearch.into(),
                magnet,
                movie_properties: Some(MovieProperties::new(
                    String::new(),
                    Quality::from(&name),
                    Codec::from(&name),
                    Source::from(&name),
                )),

                name,
            })
        }

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
                *movie_options.sort(),
                *movie_options.order(),
            );

            let mut torrents = Self::search(&options, http).await?;

            torrents.retain(|t| is_title_match(title, &t.name));

            Ok(torrents)
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Order;

    use super::*;

    #[test]
    fn test_format_url() {
        let search_options = SearchOptions::new(
            "the matrix".to_string(),
            Category::Video,
            SortColumn::Size,
            Order::Ascending,
        );

        let url = BitSearch::format_url(&search_options);

        assert!(url
            .as_str()
            .ends_with("search?q=the+matrix&sort=size&order=asc&category=1"));
    }

    #[test]
    fn test_expand_number() {
        assert_eq!(BitSearch::expand_number("1.2K"), "1200");
        assert_eq!(BitSearch::expand_number("1.2M"), "1200000");
        assert_eq!(BitSearch::expand_number("1.2B"), "1200000000");
    }

    #[test]
    fn test_format_category() {
        assert_eq!(BitSearch::format_category(&Category::All), "");
        assert_eq!(BitSearch::format_category(&Category::Applications), "5");
        assert_eq!(BitSearch::format_category(&Category::Audio), "7");
        assert_eq!(BitSearch::format_category(&Category::Video), "1");
        assert_eq!(BitSearch::format_category(&Category::Games), "6");
        assert_eq!(BitSearch::format_category(&Category::Other), "");
    }

    #[test]
    fn test_format_sort() {
        assert_eq!(BitSearch::format_sort(&SortColumn::Size), "size");
        assert_eq!(BitSearch::format_sort(&SortColumn::Seeders), "seeders");
        assert_eq!(BitSearch::format_sort(&SortColumn::Leechers), "leechers");
        assert_eq!(BitSearch::format_sort(&SortColumn::Added), "date");
    }
}
