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

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct QbittorrentConf {
    username: String,
    password: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct Config {
    qbittorrent: QbittorrentConf,
    #[serde(default = "default_movies_path")]
    movies_path: PathBuf,
}

pub fn get_config() -> Result<Config, Error> {
    let figment = figment::Figment::new()
        .merge(Env::raw().split("_"))
        .merge(Yaml::file("config.yaml"));

    let config = figment.extract()?;

    dbg!(&config);

    Ok(config)
}
