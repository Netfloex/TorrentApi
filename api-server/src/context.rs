use crate::config::Config;
use getset::Getters;
use movie_info::MovieInfoClient;
use qbittorrent_api::QbittorrentClient;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use torrent_search_client::TorrentClient;

#[derive(Getters)]
#[get = "pub"]
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

    pub async fn enable_movie_tracking(&self) {
        if !self.movie_tracking_enabled.lock().await.to_owned() {
            info!("Enabling movie progress tracking");
            *self.movie_tracking_enabled.lock().await = true;
            self.movie_tracking_ntfy.notify_waiters();
        }
    }

    pub async fn disable_movie_tracking(&self) {
        if self.movie_tracking_enabled.lock().await.to_owned() {
            info!("Disabling movie progress tracking");
            *self.movie_tracking_enabled.lock().await = false;
            self.movie_tracking_ntfy.notify_waiters();
        }
    }
}

pub type ContextPointer = Arc<Context>;
