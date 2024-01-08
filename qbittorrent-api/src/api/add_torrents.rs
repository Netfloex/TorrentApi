use surf::Body;

use crate::{
    error::ErrorKind, models::add_torrent_options::AddTorrentOptions, Error, QbittorrentClient,
};

impl QbittorrentClient {
    pub async fn add_torrents(
        &self,
        urls: &[String],
        mut options: AddTorrentOptions,
    ) -> Result<(), Error> {
        options = options.urls(urls.join("\n"));
        let body = Body::from_form(&options).unwrap();
        let resp = self
            .http
            .post("/api/v2/torrents/add")
            .body(body)
            .recv_string()
            .await?;

        if resp == "Ok." {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::TorrentAddError,
                "Could not add torrent",
            ))
        }
    }

    pub async fn add_torrent(&self, url: String, options: AddTorrentOptions) -> Result<(), Error> {
        let urls = vec![url];
        self.add_torrents(&urls, options).await
    }
}
