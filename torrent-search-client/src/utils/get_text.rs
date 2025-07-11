use surf::{Client, Url};

use crate::{Error, ErrorKind};

pub async fn get_text(url: Url, http: &Client) -> Result<String, Error> {
    let mut response = http.get(&url).send().await?;

    let status = response.status();
    if !status.is_success() {
        return Err(Error::new(
            ErrorKind::StatusCodeError(response),
            format!("Request to \"{url}\" failed with {status}"),
        ));
    }

    Ok(response.body_string().await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref TEST_CLIENT: Client = Client::new();
    }

    #[tokio::test]
    async fn test_get_text() {
        let url = Url::parse("https://httpbin.org/status/200").unwrap();
        let response = get_text(url, &TEST_CLIENT).await.unwrap();

        assert_eq!(response, "")
    }

    #[tokio::test]
    async fn test_error() {
        let url = Url::parse("https://httpbin.org/status/404").unwrap();

        let response = get_text(url, &TEST_CLIENT).await.unwrap_err();

        assert_eq!(
            response.message(),
            "Request to \"https://httpbin.org/status/404\" failed with 404"
        );
    }
}
