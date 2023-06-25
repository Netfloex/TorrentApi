use chrono::NaiveDateTime;
use derive_getters::Getters;
use serde::Serialize;

use crate::client::piratebay::PirateBayTorrent;

#[derive(Serialize, Debug, Getters)]

pub struct Torrent {
    pub added: String,
    pub category: String,
    pub file_count: usize,
    pub id: String,
    pub imdb: String,
    pub info_hash: String,
    pub leechers: usize,
    pub name: String,
    pub seeders: usize,
    pub size: u64,
    pub status: String,
    pub username: String,
    pub provider: String,
}

impl From<PirateBayTorrent> for Torrent {
    fn from(value: PirateBayTorrent) -> Self {
        Self {
            added: NaiveDateTime::parse_from_str(&value.added(), "%s")
                .unwrap_or(NaiveDateTime::default())
                .to_string(),
            category: value.category().to_owned(),
            file_count: value.num_files().parse().unwrap_or(0),
            id: value.id().to_owned(),
            imdb: value.imdb().to_owned(),
            info_hash: value.info_hash().to_owned(),
            leechers: value.leechers().parse().unwrap_or(0),
            name: value.name().to_owned(),
            seeders: value.seeders().parse().unwrap_or(0),
            size: value.size().parse().unwrap_or(0),
            status: value.status().to_owned(),
            username: value.username().to_owned(),
            provider: String::from("piratebay"),
        }
    }
}
