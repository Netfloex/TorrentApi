use crate::{
    models::{sync_main_data::SyncMainData, sync_result::SyncResult},
    Error, QbittorrentClient,
};

#[derive(serde::Serialize)]
struct Rid {
    rid: usize,
}

impl QbittorrentClient {
    pub async fn sync(&self) -> Result<SyncResult, Error> {
        let mut sync_data = self.sync_data.lock().await;

        let sync: SyncMainData = self
            .http
            .get("/api/v2/sync/maindata")
            .query(&Rid {
                rid: sync_data.sync_rid,
            })?
            .recv_json()
            .await?;

        sync_data.sync_rid = *sync.rid();

        if *sync.full_update() {
            sync_data.sync_main_data = Some(sync);
        } else {
            sync_data
                .sync_main_data
                .as_mut()
                .expect("Full update is false but sync_main_data is None.")
                .update(sync);
        }

        Ok(sync_data.sync_main_data.to_owned().unwrap().into())
    }
}
