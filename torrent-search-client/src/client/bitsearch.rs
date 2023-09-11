use crate::{
    client::Provider,
    error::Error,
    search_options::{MovieOptions, SearchOptions, SortColumn},
    torrent::Torrent,
    TorrentProvider,
};
use async_trait::async_trait;
use bytesize::ByteSize;
use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;
use reqwest::{Method, Url};
use reqwest_middleware::ClientWithMiddleware;
use scraper::{ElementRef, Html, Selector};

const BITSEARCH_API: &str = "https://bitsearch.to/search";
pub struct BitSearch {}

impl BitSearch {
    // fn format_category(category: &Category) -> &'static str {
    //     match category {
    //         Category::All => "",
    //         Category::Applications => "Apps",
    //         Category::Audio => "Music",
    //         Category::Video => "Movies",
    //         Category::Games => "Games",
    //         Category::Other => "Other",
    //     }
    // }

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
            .append_pair("order", &search_options.order().to_string());

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

        let row_selector = Selector::parse(".search-result").unwrap();

        let name_selector = Selector::parse("h5 a").unwrap();
        let magnet_selector = Selector::parse(".dl-magnet").unwrap();
        let stats_selector = Selector::parse(".stats div").unwrap();

        let info_hash_regex = Regex::new("urn:btih:([A-F\\d]+)").unwrap();

        let rows = parsed.select(&row_selector);

        fn get_text(item: Option<ElementRef>) -> String {
            item.and_then(|i| Some(i.text().collect::<Vec<&str>>().join("")))
                .unwrap_or(String::new())
                .trim()
                .to_string()
        }

        let mut torrents = vec![];

        rows.for_each(|row| {
            let mut stats = row.select(&stats_selector);
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
                .select(&magnet_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string()
                .replace("dn=%5BBitsearch.to%5D+", "dn=");

            let info_hash = info_hash_regex
                .find(&magnet)
                .unwrap()
                .as_str()
                .replace("urn:btih:", "");

            torrents.push(Torrent {
                name: row.select(&name_selector).next().unwrap().text().collect(),
                category: String::new(),
                added: date,
                file_count: 0,
                id: String::new(),
                imdb: String::new(),
                info_hash,
                leechers: leechers.parse().unwrap(),
                seeders: seeders.parse().unwrap(),
                size: size.parse::<ByteSize>().unwrap().0,
                status: String::new(),
                username: String::new(),
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
        todo!()
    }
}
