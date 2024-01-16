mod api;
mod error;
mod models;
pub use error::Error;
use http_cache_surf::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
pub use models::movie_info::MovieInfo;
use surf::{Client, Config};
use utils::surf_logging::SurfLogging;
pub struct MovieInfoClient {
    http: Client,
}

impl MovieInfoClient {
    pub fn new() -> Self {
        let config = Config::new().set_base_url("https://api.radarr.video/v1/".parse().unwrap());
        let client: Client = config.try_into().unwrap();

        Self {
            http: client
                .with(Cache(HttpCache {
                    mode: CacheMode::ForceCache,
                    manager: CACacheManager::default(),
                    options: HttpCacheOptions::default(),
                }))
                .with(SurfLogging),
        }
    }
}
