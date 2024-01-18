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
    #[serde(default)]
    torrents: HashMap<String, PartialTorrent>,
    #[serde(default)]
    torrents_removed: Vec<String>,
    #[serde(default)]
    categories: Categories,
}

impl SyncMainData {
    pub fn update(&mut self, other: Self) {
        self.rid = other.rid;
        self.full_update = other.full_update;

        other.torrents_removed().iter().for_each(|hash| {
            self.torrents.remove(hash);
        });

        other.torrents.into_iter().for_each(|(hash, torrent)| {
            if let Some(existing) = self.torrents.get_mut(&hash) {
                existing.merge(torrent);
            } else {
                self.torrents.insert(hash, torrent);
            }
        });
    }
}
