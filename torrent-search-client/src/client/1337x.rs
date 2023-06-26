use crate::{
    error::{Error, ErrorKind},
    search_options::{Category, SearchOptions, SortColumn},
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

const X1137_API: &str = "https://www.1337x.to";
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
        let mut base_url = Url::parse(X1137_API).unwrap();
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
        .join("/")
            + "/";

        base_url.set_path(&path);

        println!("{}", base_url);

        base_url
    }
}

#[async_trait]
impl TorrentProvider for X1137 {
    async fn search(
        search_options: &SearchOptions,
        http: &ClientWithMiddleware,
    ) -> Result<Vec<Torrent>, Error> {
        let url = X1137::format_url(search_options);
        println!("Request to: {}", url);
        let response = http.request(Method::GET, url).send().await?;
        let body = response.text().await?;

        let parsed = Html::parse_document(&body);

        let table_selector = Selector::parse("tbody tr").unwrap();
        let name_selector = Selector::parse(".name a:nth-child(2)").unwrap();
        let seeders_selector = Selector::parse(".seeds").unwrap();
        let leechers_selector = Selector::parse(".leeches").unwrap();
        let date_selector = Selector::parse(".coll-date").unwrap();
        let size_selector = Selector::parse(".size").unwrap();
        let username_selector = Selector::parse(".coll-5").unwrap();

        let no_results_selector = Selector::parse(".box-info > .box-info-detail > p").unwrap();

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

        let table = parsed.select(&table_selector);

        let torrents = table.map(|tr| {
            let date = get_text(&tr, &date_selector);
            let date = ordinal_regex.replace_all(&date, "").to_string();

            let date = DateTime::<Utc>::from_utc(
                NaiveDateTime::parse_from_str(&format!("{date} 00:00"), "%b. %e '%y %R")
                    .unwrap_or_default(),
                Utc,
            );

            Torrent {
                name: get_text(&tr, &name_selector),
                seeders: get_text(&tr, &seeders_selector).parse().unwrap_or(0),
                leechers: get_text(&tr, &leechers_selector).parse().unwrap_or(0),
                username: get_text(&tr, &username_selector),
                added: date,
                size: get_text(&tr, &size_selector)
                    .parse::<ByteSize>()
                    .unwrap_or(ByteSize(0))
                    .0,
                category: String::new(),
                id: get_item(&tr, &name_selector)
                    .and_then(|id| {
                        id.value()
                            .attr("href")
                            .and_then(|href| href.split('/').nth(2))
                    })
                    .unwrap_or("")
                    .to_string(),
                imdb: String::new(),
                info_hash: String::new(),
                file_count: 0,
                status: get_item(&tr, &username_selector)
                    .and_then(|item| item.value().classes().nth(1))
                    .unwrap_or("")
                    .to_string(),
                provider: String::from("1337x"),
            }
        });

        let torrents: Vec<Torrent> = torrents.collect();

        let no_results = parsed.select(&no_results_selector).next().is_some();
        if torrents.is_empty() && !no_results {
            return Err(Error::new(
                ErrorKind::ScrapingError(),
                "Could not find the table in the html response",
            ));
        }

        Ok(torrents)
    }
}
