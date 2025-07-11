mod api;
mod error;
mod models;
mod utils;
use ::utils::surf_logging::SurfLogging;
pub use error::Error;
pub use models::filters::Filters;
pub use models::movie_info::MovieInfo;
pub use models::tmdb_id::TmdbId;
use surf::{Client, Config};

#[derive(Default)]
pub struct MovieInfoClient {
    http: Client,
}

impl MovieInfoClient {
    pub fn new() -> Self {
        let config = Config::new().set_base_url("https://api.radarr.video/v1/".parse().unwrap());
        let client: Client = config.try_into().unwrap();

        Self {
            http: client.with(SurfLogging),
        }
    }
}
