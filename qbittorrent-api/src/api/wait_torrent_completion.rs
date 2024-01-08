use std::time::Duration;

use tokio::time::sleep;

use crate::{
    models::{partial_torrent::PartialTorrent, torrent_state::TorrentState},
    Error, ErrorKind, QbittorrentClient,
};

const QBITTORRENT_INFINITE: u64 = 8640000;
const DEFAULT_TIMEOUT: u64 = 5;
const MAX_TIMEOUT: u64 = 60;
const MIN_TIMEOUT: u64 = 1;

impl QbittorrentClient {
    pub async fn wait_torrent_completion(&mut self, hash: &str) -> Result<PartialTorrent, Error> {
        let mut stop = false;
        while !stop {
            let sync = self.sync().await?;

            if let Some(torrent) = sync.torrents().as_ref().and_then(|t| t.get(hash)) {
                let progress = torrent
                    .progress()
                    .expect("Progress should be available at sync");
                let eta = torrent
                    .eta()
                    .as_ref()
                    .expect("ETA should be available at sync")
                    .get()
                    .to_owned();
                let state = torrent
                    .state()
                    .as_ref()
                    .expect("State should be available at sync");

                println!(
                    "Torrent Info: Progress: {:.2}%, ETA: {} min, State: {:?}",
                    (progress * 100.0).round(),
                    eta / 60,
                    state
                );

                if progress == 1.0 {
                    return Ok(torrent.to_owned());
                }

                let should_wait = matches!(
                    state,
                    TorrentState::Allocating
                        | TorrentState::MetaDL
                        | TorrentState::QueuedDL
                        | TorrentState::CheckingDL
                        | TorrentState::CheckingResumeData
                );

                if should_wait {
                    println!(
                        "Waiting {}s for torrent to start downloading",
                        DEFAULT_TIMEOUT
                    );
                    sleep(Duration::from_secs(DEFAULT_TIMEOUT)).await;
                    continue;
                } else if eta == QBITTORRENT_INFINITE {
                    return Err(Error::new(ErrorKind::TorrentNotFound, "Torrent not found"));
                }

                let adjusted_eta = eta.min(MAX_TIMEOUT).max(MIN_TIMEOUT);

                sleep(Duration::from_secs(adjusted_eta)).await;
            } else {
                stop = true;
            }
        }

        Err(Error::new(ErrorKind::TorrentNotFound, "Torrent not found"))
    }
}
