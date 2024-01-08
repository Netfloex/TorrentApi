use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utils::{datetime, int_scalar::IntScalar};

use super::torrent_state::TorrentState;

mod option_datetime {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(datetime: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(datetime) = datetime {
            serializer.serialize_str(&datetime.to_rfc3339())
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i64::deserialize(deserializer)?;
        Ok(Some(DateTime::from_timestamp(s, 0).unwrap_or_default()))
    }
}

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct PartialTorrent {
    amount_left: Option<IntScalar<u64>>,
    completed: Option<IntScalar<u64>>,
    category: Option<String>,
    // #[serde(with = "option_datetime")]
    // completion_on: Option<DateTime<Utc>>,
    dlspeed: Option<i32>,
    downloaded: Option<IntScalar<u64>>,
    downloaded_session: Option<IntScalar<u64>>,
    eta: Option<IntScalar<u64>>,
    progress: Option<f64>,
    save_path: Option<String>,
    size: Option<IntScalar<u64>>,
    state: Option<TorrentState>,
}

impl PartialTorrent {
    pub fn merge(&mut self, other: Self) {
        self.amount_left = other.amount_left.or(self.amount_left.take());
        self.completed = other.completed.or(self.completed.take());
        self.category = other.category.or(self.category.take());
        // self.completion_on = other.completion_on.or(self.completion_on.take());
        self.dlspeed = other.dlspeed.or(self.dlspeed.take());
        self.downloaded = other.downloaded.or(self.downloaded.take());
        self.downloaded_session = other.downloaded_session.or(self.downloaded_session.take());
        self.eta = other.eta.or(self.eta.take());
        self.progress = other.progress.or(self.progress.take());
        self.save_path = other.save_path.or(self.save_path.take());
        self.size = other.size.or(self.size.take());
        self.state = other.state.or(self.state.take());
    }
}
