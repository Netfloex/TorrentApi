mod api;
mod auth_middleware;
mod error;
mod models;
mod r#static;
use auth_middleware::AuthMiddleware;
mod utils;
pub use error::Error;
pub use error::ErrorKind;
pub use models::add_category_options::AddCategoryOptions;
pub use models::add_torrent_options::AddTorrentOptions;
pub use models::category::{Categories, Category};
pub use models::get_torrents_parameters::GetTorrentsParameters;
pub use models::partial_torrent::PartialTorrent;
pub use models::sync_main_data::SyncMainData;
pub use models::torrent::Torrent;
use std::fmt::Debug;
use surf::Client;
use surf::{Config, Url};
pub struct QbittorrentClient {
    http: Client,
    sync_rid: usize,
    sync_main_data: Option<SyncMainData>,
}

impl QbittorrentClient {
    pub fn new<S: Into<String>, P: Into<String>, U: TryInto<Url>>(
        username: S,
        password: P,
        url: U,
    ) -> Self
    where
        U::Error: Debug,
    {
        let url: Url = url.try_into().expect("Invalid url");

        let config = Config::new().set_base_url(url.clone());
        let client: Client = config.try_into().unwrap();

        Self {
            http: client.with(AuthMiddleware::new(username.into(), password.into(), url)),
            sync_rid: 0,
            sync_main_data: None,
        }
    }
}
