use crate::{Error, QbittorrentClient};

impl QbittorrentClient {
    pub async fn version(&self) -> Result<String, Error> {
        let version = self.http.get("/api/v2/app/version").recv_string().await?;

        Ok(version)
    }
}
