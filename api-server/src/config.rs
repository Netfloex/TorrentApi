use derive_getters::Getters;
use figment::{
    providers::{Env, Format, Yaml},
    Error,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Getters)]
pub struct QbittorrentConf {
    username: String,
    password: String,
    url: String,
}
#[derive(Default, Debug, Serialize, Deserialize, Getters)]
pub struct Config {
    qbittorrent: QbittorrentConf,
}

pub fn get_config() -> Result<Config, Error> {
    let conf: Config = figment::Figment::new()
        .merge(Env::raw().split("_"))
        .merge(Yaml::file("config.yaml"))
        .extract()?;

    Ok(conf)
}
