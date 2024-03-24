use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Setters, Default)]
#[setters(strip_option = true)]
pub struct AddTorrentOptions {
    urls: String,
    savepath: Option<String>,
    cookie: Option<String>,
    category: Option<String>,
    tags: Option<String>,
    skip_checking: Option<bool>,
    paused: Option<bool>,
    root_folder: Option<bool>,
    rename: Option<String>,
    #[serde(rename = "upLimit")]
    up_limit: Option<i32>,
    #[serde(rename = "dlLimit")]
    dl_limit: Option<i32>,
    #[serde(rename = "autoTMM")]
    auto_tmm: Option<bool>,
    #[serde(rename = "sequentialDownload")]
    sequential_download: Option<bool>,
    #[serde(rename = "firstLastPiecePrio")]
    first_last_piece_prio: Option<bool>,
}
