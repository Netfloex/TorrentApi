use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

use crate::{Categories, Torrent};

#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct SyncMainData {
    rid: usize,
    #[serde(default)]
    full_update: bool,
    torrents: Option<HashMap<String, Torrent>>,
    torrents_removed: Option<Vec<String>>,
    categories: Option<Categories>,
}
