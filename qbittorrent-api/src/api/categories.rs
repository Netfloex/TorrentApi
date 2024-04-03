use crate::{models::category::Category, Error, QbittorrentClient};

impl QbittorrentClient {
    pub async fn categories(&self) -> Result<Vec<Category>, Error> {
        let sync = self.sync().await?;

        Ok(sync.categories().clone())
    }
}
