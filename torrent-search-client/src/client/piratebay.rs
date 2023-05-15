use crate::{http::HttpClient, TorrentProvider};
use async_trait::async_trait;
use reqwest::{Method, Url};

const PIRATE_BAY_API: &str = "https://apibay.org/q.php";
pub struct PirateBay {}

impl PirateBay {
    fn format_url(query: &str) -> String {
        let mut base_url = Url::parse(PIRATE_BAY_API).unwrap();
        base_url.query_pairs_mut().append_pair("q", query);

        base_url.to_string()
    }
}

#[async_trait]
impl TorrentProvider for PirateBay {
    async fn search(query: &str, http: &HttpClient) {
        let url = PirateBay::format_url(query);

        let response = http.request(Method::GET, url).send().await.unwrap();

        println!("{}", response.text().await.unwrap())
    }
}
