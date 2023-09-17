use chrono::{DateTime, Utc};
use derive_getters::Getters;
use juniper::GraphQLObject;
use serde::{Serialize, Serializer};
use torrent_search_client::{MovieProperties, Provider, Torrent};

use crate::int_scalar::IntScalar;

fn serialize_datetime<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.to_rfc3339())
}

#[derive(Serialize, Debug, Getters, Clone, GraphQLObject)]
pub struct ApiTorrent {
    #[serde(serialize_with = "serialize_datetime")]
    added: DateTime<Utc>,
    category: String,
    file_count: i32,
    id: String,
    info_hash: String,
    leechers: i32,
    name: String,
    seeders: i32,
    size: IntScalar<u64>,
    provider: Provider,
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
        if self.size().get() == &0 {
            self.size = other.size
        }
        if self.magnet.is_empty() {
            self.magnet = other.magnet
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
            size: IntScalar::from(value.size),
            provider: value.provider,
            magnet: value.magnet,
            movie_properties: value.movie_properties,
        }
    }
}
