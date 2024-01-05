use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Setters)]
#[setters(strip_option = true)]
pub struct AddTorrentOptions {
    urls: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    savepath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_checking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_folder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "upLimit")]
    up_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dlLimit")]
    dl_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autoTMM")]
    auto_tmm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sequentialDownload")]
    sequential_download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "firstLastPiecePrio")]
    first_last_piece_prio: Option<bool>,
}

impl AddTorrentOptions {
    pub fn new() -> Self {
        Self {
            urls: "".to_string(),
            savepath: None,
            cookie: None,
            category: None,
            tags: None,
            skip_checking: None,
            paused: None,
            root_folder: None,
            rename: None,
            up_limit: None,
            dl_limit: None,
            auto_tmm: None,
            sequential_download: None,
            first_last_piece_prio: None,
        }
    }
}
