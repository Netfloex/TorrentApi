use serde::Deserialize;
use surf::{Client, Url};

use crate::Error;

use super::get_text::get_text;

pub async fn get_json<T: for<'de> Deserialize<'de>>(url: Url, http: &Client) -> Result<T, Error> {
    let text = get_text(url, http).await?;

    let json = serde_json::from_str(&text)?;

    Ok(json)
}
