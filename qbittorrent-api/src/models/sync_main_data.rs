use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

use crate::Categories;

use super::partial_torrent::PartialTorrent;

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
pub struct SyncMainData {
    rid: usize,
    #[serde(default)]
    full_update: bool,
    torrents: Option<HashMap<String, PartialTorrent>>,
    torrents_removed: Option<Vec<String>>,
    categories: Option<Categories>,
}

impl SyncMainData {
    pub fn update(&mut self, other: Self) {
        self.rid = other.rid;
        self.full_update = other.full_update;

        if let Some(torrents) = other.torrents {
            torrents.into_iter().for_each(|(hash, torrent)| {
                let self_torrents = self.torrents.as_mut().unwrap();
                if let Some(existing) = self_torrents.get_mut(&hash) {
                    existing.merge(torrent);
                } else {
                    self_torrents.insert(hash, torrent);
                }
            });
        }

        if let Some(torrents_removed) = other.torrents_removed {
            torrents_removed.into_iter().for_each(|hash| {
                self.torrents.as_mut().unwrap().remove(&hash);
            });
        }
    }
}
