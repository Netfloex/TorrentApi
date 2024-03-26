use std::{borrow::Cow, collections::HashSet};

use crate::{
    client::{piratebay::PirateBayTorrent, yts::YtsTorrent, Provider},
    movie_properties::MovieProperties,
    r#static::trackers::{piratebay::PIRATEBAY_TRACKERS, yts::YTS_TRACKERS},
    Codec, Quality, Source,
};
use chrono::{DateTime, TimeZone, Utc};
use serde::Serialize;
use urlencoding::encode;

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct Torrent {
    pub added: DateTime<Utc>,
    pub category: String,
    pub file_count: usize,
    pub id: String,
    pub info_hash: String,
    pub leechers: usize,
    pub name: String,
    pub seeders: usize,
    pub size: u64,
    pub provider: HashSet<Provider>,
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
        if self.file_count == 0 {
            self.file_count = other.file_count;
        }
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
        if self.leechers == 0 {
            self.leechers = other.leechers;
        }
        if self.name.is_empty() {
            self.name = other.name
        }
        if self.seeders == 0 {
            self.seeders = other.seeders;
        }
        if self.size == 0 {
            self.size = other.size
        }
        if self.magnet.is_empty() {
            self.magnet = other.magnet
        }
        self.provider.extend(&other.provider)
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
            provider: Provider::PirateBay.into(),
            magnet: format_magnet(value.info_hash(), value.name(), PIRATEBAY_TRACKERS),
            movie_properties: Some(MovieProperties::new(
                value.imdb().to_owned(),
                Quality::from(value.name()),
                Codec::from(value.name()),
                Source::from(value.name()),
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
                .timestamp_opt(*torrent.date_uploaded_unix(), 0)
                .single()
                .unwrap_or_default(),
            category: String::from("movies"),
            file_count: 0,
            id: torrent.hash().to_owned(),
            info_hash: torrent.hash().to_owned(),
            leechers: torrent.peers().to_owned(),
            seeders: torrent.seeds().to_owned(),
            size: torrent.size_bytes().to_owned(),
            provider: Provider::Yts.into(),
            magnet: format_magnet(torrent.hash(), &name, YTS_TRACKERS),
            movie_properties: Some(MovieProperties::new(
                value.imdb().to_owned(),
                Quality::from(&name),
                Codec::from(&name),
                Source::from(&name),
            )),

            name,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_merge() {
        let mut torrent1 = Torrent {
            added: Utc::now(),
            category: "1".into(),
            file_count: 1,
            id: "1".into(),
            info_hash: "1".into(),
            leechers: 1,
            name: "1".into(),
            seeders: 1,
            size: 1,
            provider: Provider::PirateBay.into(),
            magnet: "1".into(),
            movie_properties: None,
        };

        let torrent2 = Torrent {
            added: Utc::now(),
            category: "2".into(),
            file_count: 2,
            id: "2".into(),
            info_hash: "2".into(),
            leechers: 2,
            name: "2".into(),
            seeders: 2,
            size: 2,
            provider: vec![Provider::Yts, Provider::PirateBay]
                .into_iter()
                .collect(),
            magnet: "2".into(),
            movie_properties: Some(MovieProperties::new(
                "2".into(),
                Quality::Unknown,
                Codec::Unknown,
                Source::Unknown,
            )),
        };

        torrent1.merge(torrent2);

        assert_eq!(torrent1.file_count, 1);
        assert_eq!(torrent1.id, "1");
        assert_eq!(torrent1.leechers, 1);
        assert_eq!(torrent1.name, "1");
        assert_eq!(torrent1.seeders, 1);
        assert_eq!(torrent1.size, 1);
        assert_eq!(
            torrent1.provider,
            vec![Provider::PirateBay, Provider::Yts]
                .into_iter()
                .collect()
        );
        assert_eq!(torrent1.magnet, "1");
        assert_eq!(
            torrent1.movie_properties.unwrap(),
            MovieProperties::new(
                "2".into(),
                Quality::Unknown,
                Codec::Unknown,
                Source::Unknown
            )
        );
    }
}
