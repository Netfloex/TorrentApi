use std::path::PathBuf;

use derive_getters::Getters;
use figment::{
    providers::{Env, Format, Yaml},
    Error,
};

use serde::{Deserialize, Serialize};

fn default_movies_path() -> PathBuf {
    PathBuf::from("./movies")
}

fn default_category() -> String {
    "torrent-api".to_string()
}

fn default_movie_tracking() -> bool {
    false
}

fn default_movie_tracking_max_timeout_active() -> u64 {
    60
}

fn default_movie_tracking_timeout_inactive() -> u64 {
    300
}

fn default_movie_tracking_min_timeout() -> u64 {
    1
}

#[derive(Debug, Serialize, Deserialize, Getters, Clone)]
pub struct QbittorrentConf {
    username: String,
    password: String,
    url: String,
    #[serde(default = "default_category")]
    category: String,
}

#[derive(Debug, Serialize, Deserialize, Getters, Clone)]
pub struct Config {
    qbittorrent: QbittorrentConf,
    remote_download_path: String,
    local_download_path: String,
    #[serde(default = "default_movies_path")]
    movies_path: PathBuf,
    #[serde(default = "default_movie_tracking")]
    disable_movie_tracking: bool,
    #[serde(default = "default_movie_tracking_max_timeout_active")]
    movie_tracking_max_timeout_active: u64,
    #[serde(default = "default_movie_tracking_timeout_inactive")]
    movie_tracking_timeout_inactive: u64,
    #[serde(default = "default_movie_tracking_min_timeout")]
    movie_tracking_min_timeout: u64,
}

pub fn get_config() -> Result<Config, Error> {
    let figment = figment::Figment::new()
        .merge(Env::raw())
        .merge(Env::raw().split("_"))
        .merge(Yaml::file("config.yaml"));

    let config = figment.extract()?;

    dbg!(&config);

    Ok(config)
}
