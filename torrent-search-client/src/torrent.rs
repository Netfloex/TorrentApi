use std::borrow::Cow;

use crate::{
    client::{piratebay::PirateBayTorrent, yts::YtsTorrent, Provider},
    movie_properties::MovieProperties,
    r#static::trackers::{PIRATEBAY_TRACKERS, YTS_TRACKERS},
    Quality, Source, VideoCodec,
};
use chrono::{DateTime, TimeZone, Utc};
use derive_getters::Getters;
use serde::Serialize;
use urlencoding::encode;

#[derive(Serialize, Debug, Getters, Clone)]
pub struct Torrent {
    pub added: DateTime<Utc>,
    pub category: String,
    pub file_count: i32,
    pub id: String,
    pub info_hash: String,
    pub leechers: i32,
    pub name: String,
    pub seeders: i32,
    pub size: u64,
    pub provider: Provider,
    pub magnet: String,
    pub movie_properties: Option<MovieProperties>,
}

impl Torrent {
    pub fn merge(&mut self, other: Self) {
        if self.added.timestamp_millis() == 0 {
            self.added = other.added
        };
        if self.category.is_empty() {
            self.category = other.category
        }
        self.file_count |= other.file_count;
        if self.id.is_empty() {
            self.id = other.id
        }
        if let Some(props) = self.movie_properties.as_mut() {
            if let Some(other_props) = other.movie_properties {
                props.merge(other_props);
            }
        } else {
            self.movie_properties = other.movie_properties
        }
        self.leechers |= other.leechers;
        if self.name.is_empty() {
            self.name = other.name
        }
        self.seeders |= other.seeders;
        self.size |= other.size;
        if self.magnet.is_empty() {
            self.magnet = other.magnet
        }
    }
}

fn format_magnet(hash: &str, name: &str, trackers: &[&str]) -> String {
    let trackers = trackers
        .iter()
        .map(|tr| encode(tr))
        .collect::<Vec<Cow<str>>>()
        .join("&tr=");

    format!(
        "magnet:?xt=urn:btih:{}&tr={}&dn={}",
        hash,
        trackers,
        encode(name)
    )
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
            info_hash: value.info_hash().to_owned(),
            leechers: value.leechers().parse().unwrap_or(0),
            name: value.name().to_string(),
            seeders: value.seeders().parse().unwrap_or(0),
            size: value.size().parse().unwrap_or(0),
            provider: Provider::PirateBay,
            magnet: format_magnet(value.info_hash(), value.name(), PIRATEBAY_TRACKERS),
            movie_properties: Some(MovieProperties::new(
                value.imdb().to_owned(),
                Quality::from_str(value.name()),
                VideoCodec::from_str(value.name()),
                Source::from_str(value.name()),
            )),
        }
    }
}

impl From<YtsTorrent> for Torrent {
    fn from(value: YtsTorrent) -> Self {
        let torrent = value.data();
        let name = format!(
            "{} [{}] [{}] {}",
            value.title(),
            torrent.quality(),
            torrent.source(),
            torrent.video_codec()
        );

        Self {
            added: Utc
                .timestamp_opt(torrent.date_uploaded_unix().to_owned(), 0)
                .single()
                .unwrap_or_default(),
            category: String::from("movies"),
            file_count: 0,
            id: torrent.hash().to_owned(),
            info_hash: torrent.hash().to_owned(),
            leechers: torrent.peers().to_owned(),
            name: name.clone(),
            seeders: torrent.seeds().to_owned(),
            size: torrent.size_bytes().to_owned(),
            provider: Provider::Yts,
            magnet: format_magnet(torrent.hash(), &name, YTS_TRACKERS),
            movie_properties: Some(MovieProperties::new(
                value.imdb().to_owned(),
                Quality::from_str(&name),
                VideoCodec::from_str(&name),
                Source::from_str(&name),
            )),
        }
    }
}
