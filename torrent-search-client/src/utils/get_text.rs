use surf::{Client, Url};

use crate::{Error, ErrorKind};

pub async fn get_text(url: Url, http: &Client) -> Result<String, Error> {
    let mut response = http.get(&url).send().await?;

    let status = response.status();
    if !status.is_success() {
        return Err(Error::new(
            ErrorKind::StatusCodeError(response),
            format!("Request to \"{}\" failed with {}", url, status),
        ));
    }

    Ok(response.body_string().await?)
}
