use crate::client::{piratebay::PirateBayTorrent, yts::YtsTorrent, Provider};
use chrono::{DateTime, TimeZone, Utc};
use derive_getters::Getters;
use serde::{Serialize, Serializer};
use urlencoding::encode;

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
    pub provider: Provider,
    pub magnet: String,
}

fn format_magnet(hash: &str, name: &str) -> String {
    format!("magnet:?xt=urn:btih:{}&dn={}", hash, encode(name))
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
            provider: Provider::PirateBay,
            magnet: format_magnet(value.info_hash(), value.name()),
        }
    }
}

impl From<YtsTorrent> for Torrent {
    fn from(value: YtsTorrent) -> Self {
        let unsupported = String::from("unsupported");
        let torrent = value.data();
        let name = format!(
            "{} [{}] [{}] {}",
            value.title(),
            torrent.quality(),
            torrent.kind(),
            torrent.video_codec()
        );
        Self {
            added: Utc
                .timestamp_opt(torrent.date_uploaded_unix().to_owned(), 0)
                .single()
                .unwrap_or_default(),
            category: String::from("movies"),
            file_count: 1,
            id: torrent.hash().to_owned(),
            imdb: value.imdb().to_owned(),
            info_hash: torrent.hash().to_owned(),
            leechers: torrent.peers().to_owned(),
            name: name.clone(),
            seeders: torrent.seeds().to_owned(),
            size: torrent.size_bytes().to_owned(),
            status: unsupported.clone(),
            username: unsupported.clone(),
            provider: Provider::Yts,
            magnet: format_magnet(torrent.hash(), &name),
        }
    }
}
