use reqwest::{Method, Url};
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

use crate::Error;

pub async fn get_json<T: for<'de> Deserialize<'de>>(
    url: Url,
    http: &ClientWithMiddleware,
) -> Result<T, Error> {
    let response = http
        .request(Method::GET, url)
        .send()
        .await?
        .error_for_status()?;

    let json: T = response.json().await?;

    Ok(json)
}
