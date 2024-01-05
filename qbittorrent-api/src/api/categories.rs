use crate::{
    models::category::{Categories, Category},
    Error, QbittorrentClient,
};

impl QbittorrentClient {
    pub async fn categories(&self) -> Result<Vec<Category>, Error> {
        let resp: Categories = self
            .http
            .get("/api/v2/torrents/categories")
            .recv_json()
            .await?;

        Ok(resp.categories())
    }
}
