use super::torrent_state::TorrentState;
use chrono::{serde::ts_seconds::deserialize as ts_seconds, DateTime, Utc};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[get = "pub with_prefix"]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct Torrent {
    #[serde(deserialize_with = "ts_seconds")]
    added_on: DateTime<Utc>,
    amount_left: u64,
    auto_tmm: bool,
    availability: f64,
    category: String,
    completed: u64,
    #[serde(deserialize_with = "ts_seconds")]
    completion_on: DateTime<Utc>,
    content_path: String,
    dl_limit: i32,
    dlspeed: i32,
    downloaded: u64,
    downloaded_session: u64,
    eta: u64,
    f_l_piece_prio: bool,
    force_start: bool,
    hash: String,
    #[serde(deserialize_with = "ts_seconds")]
    last_activity: DateTime<Utc>,
    magnet_uri: String,
    max_ratio: f64,
    max_seeding_time: i32,
    name: String,
    num_complete: i32,
    num_incomplete: i32,
    num_leechs: i32,
    num_seeds: i32,
    priority: i32,
    progress: f64,
    ratio: f64,
    ratio_limit: f64,
    save_path: String,
    seeding_time_limit: i32,
    #[serde(deserialize_with = "ts_seconds")]
    seen_complete: DateTime<Utc>,
    seq_dl: bool,
    size: u64,
    state: TorrentState,
    super_seeding: bool,
    tags: String,
    time_active: i32,
    total_size: i64,
    tracker: String,
    up_limit: i32,
    uploaded: u64,
    uploaded_session: u64,
    upspeed: i32,
}
