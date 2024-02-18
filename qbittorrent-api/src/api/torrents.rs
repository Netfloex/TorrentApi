use crate::{error::ErrorKind, Error, GetTorrentsParameters, QbittorrentClient, Torrent};
use log::error;

impl QbittorrentClient {
    pub async fn torrents(&self, options: GetTorrentsParameters) -> Result<Vec<Torrent>, Error> {
        let mut resp = self
            .http
            .get("/api/v2/torrents/info")
            .query(&options)?
            .await?;

        if resp.status().is_success() {
            let text = resp.body_string().await?;

            let json = serde_json::from_str::<Vec<Torrent>>(&text);
            if let Err(error) = json {
                error!("Serde error");
                dbg!(text);
                dbg!(&error);
                Err(error)?
            } else {
                Ok(json?)
            }
        } else {
            let body = resp.body_string().await?;
            if body.ends_with("parameter is invalid") {
                Err(Error::new(
                    ErrorKind::BadParameters(body.replace(" parameter is invalid", "")),
                    "message",
                ))
            } else {
                Err(Error::new(ErrorKind::RequestError, body))
            }
        }
    }
}
