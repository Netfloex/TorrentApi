use chrono::NaiveDateTime;
use serde::Serialize;

use crate::client::piratebay::PirateBayTorrent;

#[derive(Serialize, Debug)]

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
            added: NaiveDateTime::parse_from_str(&value.added, "%s")
                .unwrap()
                .to_string(),
            category: value.category,
            file_count: value.num_files.parse().unwrap_or(0),
            id: value.id,
            imdb: value.imdb,
            info_hash: value.info_hash,
            leechers: value.leechers.parse().unwrap_or(0),
            name: value.name,
            seeders: value.seeders.parse().unwrap_or(0),
            size: value.size.parse().unwrap_or(0),
            status: value.status,
            username: value.username,
            provider: String::from("piratebay"),
        }
    }
}
