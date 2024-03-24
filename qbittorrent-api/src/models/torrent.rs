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
    amount_left: usize,
    auto_tmm: bool,
    availability: f64,
    category: String,
    completed: usize,
    #[serde(deserialize_with = "ts_seconds")]
    completion_on: DateTime<Utc>,
    content_path: String,
    dl_limit: i32,
    dlspeed: u32,
    downloaded: usize,
    downloaded_session: usize,
    eta: usize,
    f_l_piece_prio: bool,
    force_start: bool,
    hash: String,
    #[serde(deserialize_with = "ts_seconds")]
    last_activity: DateTime<Utc>,
    magnet_uri: String,
    max_ratio: f64,
    max_seeding_time: i32,
    name: String,
    num_complete: usize,
    num_incomplete: usize,
    num_leechs: usize,
    num_seeds: usize,
    priority: i32,
    progress: f64,
    ratio: f64,
    ratio_limit: f64,
    save_path: String,
    seeding_time_limit: i32,
    #[serde(deserialize_with = "ts_seconds")]
    seen_complete: DateTime<Utc>,
    seq_dl: bool,
    size: usize,
    state: TorrentState,
    super_seeding: bool,
    tags: String,
    time_active: usize,
    total_size: usize,
    tracker: String,
    up_limit: i32,
    uploaded: usize,
    uploaded_session: usize,
    upspeed: u32,
}
