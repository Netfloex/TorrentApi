use getset::Getters;

use crate::{Category, SyncMainData, Torrent};

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct SyncResult {
    torrents: Vec<Torrent>,
    categories: Vec<Category>,
}

impl From<SyncMainData> for SyncResult {
    fn from(sync_main_data: SyncMainData) -> Self {
        Self {
            torrents: sync_main_data
                .torrents()
                .values()
                .cloned()
                .map(|torrent| serde_json::from_value(torrent).unwrap())
                .collect(),
            categories: sync_main_data
                .categories()
                .values()
                .cloned()
                .map(|category| serde_json::from_value(category).unwrap())
                .collect(),
        }
    }
}
