use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utils::int_scalar::IntScalar;

use super::torrent_state::TorrentState;

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct PartialTorrent {
    amount_left: Option<IntScalar<u64>>,
    category: Option<String>,
    completed: Option<IntScalar<u64>>,
    content_path: Option<String>,
    dlspeed: Option<i32>,
    downloaded_session: Option<IntScalar<u64>>,
    downloaded: Option<IntScalar<u64>>,
    eta: Option<IntScalar<u64>>,
    name: Option<String>,
    progress: Option<f64>,
    size: Option<IntScalar<u64>>,
    state: Option<TorrentState>,
}

impl PartialTorrent {
    pub fn merge(&mut self, other: Self) {
        self.amount_left = other.amount_left.or(self.amount_left.take());
        self.category = other.category.or(self.category.take());
        self.completed = other.completed.or(self.completed.take());
        self.content_path = other.content_path.or(self.content_path.take());
        self.dlspeed = other.dlspeed.or(self.dlspeed.take());
        self.downloaded = other.downloaded.or(self.downloaded.take());
        self.downloaded_session = other.downloaded_session.or(self.downloaded_session.take());
        self.eta = other.eta.or(self.eta.take());
        self.name = other.name.or(self.name.take());
        self.progress = other.progress.or(self.progress.take());
        self.size = other.size.or(self.size.take());
        self.state = other.state.or(self.state.take());
    }
}
