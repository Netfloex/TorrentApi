use chrono::{DateTime, Utc};
use derive_getters::Getters;
use juniper::GraphQLObject;
use serde::Serialize;
use torrent_search_client::{MovieProperties, Provider, Torrent};
use utils::int_scalar::IntScalar;

#[derive(Serialize, Debug, Getters, Clone, GraphQLObject)]
pub struct ApiTorrent {
    added: DateTime<Utc>,
    category: String,
    file_count: i32,
    id: String,
    info_hash: String,
    leechers: i32,
    name: String,
    seeders: i32,
    size: IntScalar<u64>,
    provider: Vec<Provider>,
    magnet: String,
    movie_properties: Option<MovieProperties>,
}

impl ApiTorrent {
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
        if **self.size() == 0 {
            self.size = other.size
        }
        if self.magnet.is_empty() {
            self.magnet = other.magnet
        }
        for provider in other.provider {
            if !self.provider.contains(&provider) {
                self.provider.push(provider);
            }
        }
    }
}

impl From<Torrent> for ApiTorrent {
    fn from(value: Torrent) -> Self {
        Self {
            added: value.added,
            category: value.category,
            file_count: value.file_count,
            id: value.id,
            info_hash: value.info_hash,
            leechers: value.leechers,
            name: value.name,
            seeders: value.seeders,
            size: value.size.into(),
            provider: vec![value.provider],
            magnet: value.magnet,
            movie_properties: value.movie_properties,
        }
    }
}

#[cfg(test)]
mod tests {
    use torrent_search_client::{Codec, Quality, Source};

    use super::*;

    #[test]
    fn test_merge() {
        let mut torrent1 = ApiTorrent {
            added: Utc::now(),
            category: "1".into(),
            file_count: 1,
            id: "1".into(),
            info_hash: "1".into(),
            leechers: 1,
            name: "1".into(),
            seeders: 1,
            size: IntScalar::from(1),
            provider: vec![Provider::PirateBay],
            magnet: "1".into(),
            movie_properties: None,
        };

        let torrent2 = ApiTorrent {
            added: Utc::now(),
            category: "2".into(),
            file_count: 2,
            id: "2".into(),
            info_hash: "2".into(),
            leechers: 2,
            name: "2".into(),
            seeders: 2,
            size: IntScalar::from(2),
            provider: vec![Provider::Yts, Provider::PirateBay],
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
        assert_eq!(**torrent1.size(), 1);
        assert_eq!(torrent1.provider, vec![Provider::PirateBay, Provider::Yts]);
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
