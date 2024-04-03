use getset::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[get = "pub"]
pub struct SyncMainData {
    rid: usize,
    #[serde(default)]
    full_update: bool,
    #[serde(default)]
    torrents: HashMap<String, Value>,
    #[serde(default)]
    torrents_removed: Vec<String>,
    #[serde(default)]
    categories: HashMap<String, Value>,
    #[serde(default)]
    categories_removed: Vec<String>,
}

fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b.clone(),
    }
}

impl SyncMainData {
    pub fn update(&mut self, other: Self) {
        self.rid = other.rid;
        self.full_update = other.full_update;

        other.torrents_removed.iter().for_each(|hash| {
            self.torrents.remove(hash);
        });

        other.torrents.into_iter().for_each(|(hash, torrent)| {
            if let Some(existing) = self.torrents.get_mut(&hash) {
                merge(existing, torrent);
            } else {
                self.torrents.insert(hash, torrent);
            }
        });

        other.categories_removed.iter().for_each(|category| {
            self.categories.remove(category);
        });

        other.categories.into_iter().for_each(|(category, value)| {
            if let Some(existing) = self.categories.get_mut(&category) {
                merge(existing, value);
            } else {
                self.categories.insert(category, value);
            }
        });
    }
}
