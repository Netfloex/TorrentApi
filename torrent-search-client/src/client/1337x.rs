use crate::{category::Category, torrent::Torrent, TorrentProvider};
use async_trait::async_trait;
use bytesize::ByteSize;
use chrono::NaiveDateTime;
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

    fn format_url(query: &str, category: &Category) -> Url {
        let mut base_url = Url::parse(X1137_API).unwrap();
        let has_category = !matches!(category, Category::All);

        let path = vec![
            if has_category {
                "category-search"
            } else {
                "search"
            },
            query,
            if has_category {
                Self::format_category(category)
            } else {
                ""
            },
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
    async fn search(query: &str, category: &Category, http: &ClientWithMiddleware) -> Vec<Torrent> {
        let url = X1137::format_url(query, category);
        println!("Request to: {}", url);
        let response = http.request(Method::GET, url).send().await.unwrap();
        println!("Status: {}", response.status());
        let body = response.text().await.unwrap();

        let parsed = Html::parse_document(&body);

        let table_selector = Selector::parse("tbody tr").unwrap();
        let name_selector = Selector::parse(".name a:nth-child(2)").unwrap();
        let seeders_selector = Selector::parse(".seeds").unwrap();
        let leechers_selector = Selector::parse(".leeches").unwrap();
        let date_selector = Selector::parse(".coll-date").unwrap();
        let size_selector = Selector::parse(".size").unwrap();
        let username_selector = Selector::parse(".coll-5").unwrap();

        fn get_item<'a>(tr: ElementRef<'a>, selector: &'a Selector) -> ElementRef<'a> {
            tr.select(&selector).next().unwrap()
        }

        fn get_text(tr: ElementRef, selector: &Selector) -> String {
            get_item(tr, selector).text().next().unwrap().to_string()
        }

        let ordinal_regex = Regex::new(r#"st|nd|rd|th"#).unwrap();

        let torrents = parsed.select(&table_selector).map(|tr| {
            let date = get_text(tr, &date_selector);
            let date = ordinal_regex.replace_all(&date, "").to_string();

            let date = NaiveDateTime::parse_from_str(&format!("{date} 00:00"), "%b. %e '%y %R");

            match date {
                Err(e) => println!("{}: {}", e, get_text(tr, &date_selector)),
                Ok(_) => (),
            }

            Torrent {
                name: get_text(tr, &name_selector),
                seeders: get_text(tr, &seeders_selector).parse().unwrap_or(0),
                leechers: get_text(tr, &leechers_selector).parse().unwrap_or(0),
                username: get_text(tr, &username_selector),
                added: match date {
                    Ok(t) => t.to_string(),
                    Err(err) => err.to_string(),
                },
                size: get_text(tr, &size_selector).parse::<ByteSize>().unwrap().0,
                category: String::new(),
                id: get_item(tr, &name_selector)
                    .value()
                    .attr("href")
                    .unwrap()
                    .split("/")
                    .nth(2)
                    .unwrap()
                    .to_string(),
                imdb: String::new(),
                info_hash: String::new(),
                file_count: 0,
                status: get_item(tr, &username_selector)
                    .value()
                    .classes()
                    .nth(1)
                    .unwrap()
                    .to_string(),
                provider: String::from("1337x"),
            }
        });

        torrents.collect()
    }
}
