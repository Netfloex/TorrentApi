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

fn default_progress_check() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct QbittorrentConf {
    username: String,
    password: String,
    url: String,
    #[serde(default = "default_category")]
    category: String,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct Config {
    qbittorrent: QbittorrentConf,
    #[serde(default = "default_movies_path")]
    movies_path: PathBuf,
    #[serde(default = "default_progress_check")]
    disable_progress_check: bool,
}

pub fn get_config() -> Result<Config, Error> {
    let figment = figment::Figment::new()
        .merge(Env::raw().split("_"))
        .merge(Yaml::file("config.yaml"));

    let config = figment.extract()?;

    dbg!(&config);

    Ok(config)
}
