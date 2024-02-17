use serde::Deserialize;
use surf::{Client, Url};

use crate::Error;

use super::get_text::get_text;

pub async fn get_json<T: for<'de> Deserialize<'de>>(url: Url, http: &Client) -> Result<T, Error> {
    let text = get_text(url, http).await?;

    let json = serde_json::from_str(&text)?;

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref TEST_CLIENT: Client = Client::new();
    }

    #[tokio::test]
    async fn test_json() {
        #[derive(Deserialize)]
        struct IpData {
            pub origin: String,
        }

        let url = Url::parse("https://httpbin.org/ip").unwrap();
        let response: IpData = get_json(url, &TEST_CLIENT).await.unwrap();

        assert!(!response.origin.is_empty())
    }
}
