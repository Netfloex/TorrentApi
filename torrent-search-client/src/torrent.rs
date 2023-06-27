use chrono::{DateTime, TimeZone, Utc};
use derive_getters::Getters;
use serde::{Serialize, Serializer};

use crate::client::{piratebay::PirateBayTorrent, yts::YtsTorrent};

fn serialize_datetime<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.to_rfc3339())
}

#[derive(Serialize, Debug, Getters)]
pub struct Torrent {
    #[serde(serialize_with = "serialize_datetime")]
    pub added: DateTime<Utc>,
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
            added: Utc
                .timestamp_opt(value.added().parse().unwrap_or_default(), 0)
                .single()
                .unwrap_or_default(),
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

impl From<YtsTorrent> for Torrent {
    fn from(value: YtsTorrent) -> Self {
        let unsupported = String::from("unsupported");

        Self {
            added: Utc
                .timestamp_opt(value.date_uploaded_unix().to_owned(), 0)
                .single()
                .unwrap_or_default(),
            category: String::from("movies"),
            file_count: 1,
            id: value.hash().to_owned(),
            imdb: value.imdb().to_owned(),
            info_hash: value.hash().to_owned(),
            leechers: value.peers().to_owned(),
            name: format!(
                "{} [{}] [{}] {}",
                value.title(),
                value.quality(),
                value.kind(),
                value.video_codec()
            ),
            seeders: value.seeds().to_owned(),
            size: value.size_bytes().to_owned(),
            status: unsupported.clone(),
            username: unsupported.clone(),
            provider: String::from("yts"),
        }
    }
}
