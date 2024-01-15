use derive_getters::Getters;
use movie_info::MovieInfoClient;
use qbittorrent_api::QbittorrentClient;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use torrent_search_client::TorrentClient;

use crate::config::Config;

#[derive(Getters)]
pub struct Context {
    torrent_client: TorrentClient,
    qbittorrent_client: QbittorrentClient,
    movie_info_client: MovieInfoClient,
    config: Config,
    movie_tracking_enabled: Mutex<bool>,
    movie_tracking_ntfy: Arc<Notify>,
}

impl Context {
    pub fn new(
        torrent_client: TorrentClient,
        qbittorrent_client: QbittorrentClient,
        config: Config,
    ) -> Self {
        Self {
            torrent_client,
            qbittorrent_client,
            movie_info_client: MovieInfoClient::new(),
            config,
            movie_tracking_enabled: Mutex::new(true),
            movie_tracking_ntfy: Arc::new(Notify::new()),
        }
    }

    pub async fn enable_movie_tracking(&mut self) {
        if !self.movie_tracking_enabled.lock().await.to_owned() {
            println!("Enabling movie progress tracking");
            *self.movie_tracking_enabled.lock().await = true;
            self.movie_tracking_ntfy.notify_waiters();
        }
    }

    pub async fn disable_movie_tracking(&mut self) {
        if self.movie_tracking_enabled.lock().await.to_owned() {
            println!("Disabling movie progress tracking");
            *self.movie_tracking_enabled.lock().await = false;
            self.movie_tracking_ntfy.notify_waiters();
        }
    }

    pub fn qbittorrent_client_mut(&mut self) -> &mut QbittorrentClient {
        &mut self.qbittorrent_client
    }
}

pub type ContextPointer = Arc<Mutex<Context>>;
