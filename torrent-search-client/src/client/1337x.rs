use crate::utils::get_text;
use crate::{
    client::Provider,
    error::{Error, ErrorKind},
    search_options::{
        category::Category, movie_options::MovieOptions, sort_column::SortColumn, SearchOptions,
    },
    torrent::Torrent,
    utils::round_robin::RoundRobin,
    TorrentProvider,
};
use async_trait::async_trait;
use bytesize::ByteSize;
use chrono::{NaiveDateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::sync::Mutex;
use surf::{Client, Url};

const X1137_APIS: [&str; 1] = [
    "https://1337x.to",
    // "https://1337x.so", // cloudflare
];

lazy_static! {
    static ref TABLE_SELECTOR: Selector = Selector::parse("tbody tr").unwrap();
    static ref NAME_SELECTOR: Selector = Selector::parse(".name a:nth-child(2)").unwrap();
    static ref SEEDERS_SELECTOR: Selector = Selector::parse(".seeds").unwrap();
    static ref LEECHERS_SELECTOR: Selector = Selector::parse(".leeches").unwrap();
    static ref DATE_SELECTOR: Selector = Selector::parse(".coll-date").unwrap();
    static ref SIZE_SELECTOR: Selector = Selector::parse(".size").unwrap();
    static ref NO_RESULTS_SELECTOR: Selector =
        Selector::parse(".box-info > .box-info-detail > p").unwrap();
    static ref ROUND_ROBIN: Mutex<RoundRobin> = Mutex::new(RoundRobin::new(
        X1137_APIS.map(|url| url.parse::<Url>().unwrap()).to_vec()
    ));
}

pub struct X1137 {}
impl X1137 {
    fn format_category(category: &Category) -> &'static str {
        match category {
            Category::All => "",
            Category::Applications => "Apps",
            Category::Audio => "Music",
            Category::Video => "Movies",
            Category::Games => "Games",
            Category::Other => "Other",
        }
    }

    fn format_sort(column: &SortColumn) -> &str {
        match column {
            SortColumn::Added => "time",
            SortColumn::Leechers => "leechers",
            SortColumn::Size => "size",
            SortColumn::Seeders => "seeders",
        }
    }

    fn format_url(search_options: &SearchOptions) -> Url {
        let mut url = ROUND_ROBIN.lock().unwrap().get();

        let has_category = !matches!(search_options.category(), Category::All);

        let path = vec![
            if has_category {
                "sort-category-search"
            } else {
                "sort-search"
            },
            search_options.query(),
            if has_category {
                Self::format_category(search_options.category())
            } else {
                ""
            },
            Self::format_sort(search_options.sort()),
            &search_options.order().to_string(),
            "1",
        ]
        .into_iter()
        .filter(|i| !i.is_empty())
        .collect::<Vec<&str>>()
        .join("/")
            + "/";

        url.set_path(&path);

        url
    }
}

#[async_trait]
impl TorrentProvider for X1137 {
    const PROVIDER: Provider = Provider::X1337;

    async fn search(search_options: &SearchOptions, http: &Client) -> Result<Vec<Torrent>, Error> {
        let url = X1137::format_url(search_options);
        let body = get_text::get_text(url, http).await?;

        let parsed = Html::parse_document(&body);

        fn get_item<'a>(tr: &'a ElementRef<'a>, selector: &'a Selector) -> Option<ElementRef<'a>> {
            tr.select(selector).next()
        }

        fn get_text<'a>(tr: &ElementRef<'a>, selector: &'a Selector) -> String {
            get_item(tr, selector)
                .and_then(|i| i.text().next())
                .unwrap_or("")
                .trim()
                .to_string()
        }

        let ordinal_regex = Regex::new(r#"st|nd|rd|th"#).unwrap();

        let table = parsed.select(&TABLE_SELECTOR);

        let torrents = table.map(|tr| {
            let date = get_text(&tr, &DATE_SELECTOR);
            let date = ordinal_regex.replace_all(&date, "").to_string();

            let date = NaiveDateTime::parse_from_str(&format!("{date} 00:00"), "%b. %e '%y %R")
                .unwrap_or_default()
                .and_local_timezone(Utc)
                .unwrap();

            Torrent {
                name: get_text(&tr, &NAME_SELECTOR),
                seeders: get_text(&tr, &SEEDERS_SELECTOR).parse().unwrap_or(0),
                leechers: get_text(&tr, &LEECHERS_SELECTOR).parse().unwrap_or(0),
                added: date,
                size: get_text(&tr, &SIZE_SELECTOR)
                    .parse::<ByteSize>()
                    .unwrap_or(ByteSize(0))
                    .0,
                category: String::new(),
                id: get_item(&tr, &NAME_SELECTOR)
                    .and_then(|id| {
                        id.value()
                            .attr("href")
                            .and_then(|href| href.split('/').nth(2))
                    })
                    .unwrap_or("")
                    .to_string(),
                info_hash: String::from("unsupported"),
                file_count: 0,
                provider: Provider::X1337,
                magnet: String::from("unsupported"),
                movie_properties: None,
            }
        });

        let torrents: Vec<Torrent> = torrents.collect();

        let no_results = parsed.select(&NO_RESULTS_SELECTOR).next().is_some();
        if torrents.is_empty() && !no_results {
            return Err(Error::new(
                ErrorKind::ScrapingError(),
                "Could not find the table in the html response",
            ));
        }

        Ok(torrents)
    }

    async fn search_movie(
        _movie_options: &MovieOptions,
        _http: &Client,
    ) -> Result<Vec<Torrent>, Error> {
        todo!()
    }
}
