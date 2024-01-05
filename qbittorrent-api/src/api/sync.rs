use crate::{models::sync_main_data::SyncMainData, Error, QbittorrentClient};

#[derive(serde::Serialize)]
struct Rid {
    rid: usize,
}

impl QbittorrentClient {
    pub async fn sync(&mut self) -> Result<SyncMainData, Error> {
        let sync: SyncMainData = self
            .http
            .get("/api/v2/sync/maindata")
            .query(&Rid { rid: self.sync_rid })?
            .recv_json()
            .await?;
        self.sync_rid = sync.rid().to_owned();
        Ok(sync)
    }
}
