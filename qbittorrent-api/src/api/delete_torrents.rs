use surf::Body;

use crate::{
    models::delete_torrents_parameters::DeleteTorrentsParameters, Error, ErrorKind,
    QbittorrentClient,
};

impl QbittorrentClient {
    pub async fn delete_torrents(
        &self,
        hashes: Vec<String>,
        delete_files: bool,
    ) -> Result<(), Error> {
        let body = Body::from_form(&DeleteTorrentsParameters::new(hashes, delete_files))?;

        let mut resp = self
            .http
            .post("/api/v2/torrents/delete")
            .body(body)
            .send()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::RequestError,
                resp.body_string().await?,
            ))
        }
    }

    pub async fn delete_torrent(&self, hash: String, delete_files: bool) -> Result<(), Error> {
        self.delete_torrents(vec![hash], delete_files).await
    }
}
