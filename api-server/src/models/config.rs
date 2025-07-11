use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use figment::{
    providers::{Env, Format, Yaml},
    Error,
};
use getset::Getters;
use log::{debug, error};
use movie_info::Filters;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use super::serde_regex::SerdeRegex;

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Getters, Clone)]
#[get = "pub"]
pub struct QbittorrentConf {
    username: String,
    password: String,
    url: String,
    #[serde_inline_default("torrent-api".to_string())]
    category: String,
}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct Config {
    qbittorrent: QbittorrentConf,
    remote_download_path: String,
    local_download_path: String,
    #[serde_inline_default(vec!["US".to_string()].into_iter().collect())]
    languages: HashSet<String>,
    movies_path: PathBuf,

    #[serde_inline_default(false)]
    disable_movie_tracking: bool,

    #[serde_inline_default(60)]
    movie_tracking_max_timeout_active: usize,

    #[serde_inline_default(3600)]
    movie_tracking_timeout_inactive: usize,

    #[serde_inline_default(1)]
    movie_tracking_min_timeout: usize,

    #[serde_inline_default(false)]
    delete_torrent_after_import: bool,

    #[serde_inline_default(false)]
    delete_torrent_files: bool,

    #[serde_inline_default(String::new())]
    category_after_import: String,

    #[serde_inline_default(true)]
    hide_movies_no_imdb: bool,

    #[serde_inline_default(30)]
    hide_movies_below_runtime: u16,

    #[serde_inline_default(2)]
    import_movie_max_depth: u8,

    #[serde(default)]
    subtitle_language_map: HashMap<String, SerdeRegex>,
}

impl Config {
    pub fn filters(&self) -> Filters {
        Filters::new(
            *self.hide_movies_no_imdb(),
            *self.hide_movies_below_runtime(),
            self.languages().iter().cloned().collect(),
        )
    }
}

pub fn get_config() -> Result<Config, Error> {
    let figment = figment::Figment::new()
        .merge(Env::raw())
        .merge(Env::raw().split("_"))
        .merge(Yaml::file("config.yaml"));

    let config: Config = figment.extract()?;

    // category != category_after_import
    if config.category_after_import() == config.qbittorrent().category() {
        error!("category_after_import cannot be the same as category");
        std::process::exit(1);
    }

    // !delete_torrent_after_import && delete_torrent_files
    if !config.delete_torrent_after_import() && *config.delete_torrent_files() {
        error!("delete_torrent_files cannot be true if delete_torrent_after_import is false");
        std::process::exit(1);
    }

    debug!("{config:#?}");

    Ok(config)
}
