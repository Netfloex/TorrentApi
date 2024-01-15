mod api;
mod error;
mod models;
pub use error::Error;
pub use models::movie_info::MovieInfo;
use surf::{Client, Config};
use utils::surf_logging::SurfLogging;
pub struct MovieInfoClient {
    http: Client,
}

impl MovieInfoClient {
    pub fn new() -> Self {
        let config =
            Config::new().set_base_url("https://api.radarr.video/v1/movie/".parse().unwrap());
        let client: Client = config.try_into().unwrap();

        Self {
            http: client.with(SurfLogging),
        }
    }
}
