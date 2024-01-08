use crate::{models::sync_main_data::SyncMainData, Error, QbittorrentClient};

#[derive(serde::Serialize)]
struct Rid {
    rid: usize,
}

impl QbittorrentClient {
    pub async fn sync(&mut self) -> Result<&SyncMainData, Error> {
        let sync: String = self
            .http
            .get("/api/v2/sync/maindata")
            .query(&Rid { rid: self.sync_rid })?
            .recv_string()
            .await?;

        let sync: SyncMainData = serde_json::from_str(&sync).unwrap();

        self.sync_rid = sync.rid().to_owned();

        if *sync.full_update() {
            self.sync_main_data = Some(sync);
        } else {
            self.sync_main_data
                .as_mut()
                .expect("Full update is false but sync_main_data is None.")
                .update(sync);
        }

        Ok(self.sync_main_data.as_ref().unwrap())
    }
}
