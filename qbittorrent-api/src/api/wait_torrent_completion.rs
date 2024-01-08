use std::time::Duration;

use tokio::time::sleep;

use crate::{Error, QbittorrentClient};

const QBITTORRENT_INFINITE: u64 = 8640000;
const DEFAULT_TIMEOUT: u64 = 10;
const MAX_TIMEOUT: u64 = 60;
const MIN_TIMEOUT: u64 = 1;

impl QbittorrentClient {
    pub async fn wait_torrent_completion(&mut self, hash: &str) -> Result<bool, Error> {
        let mut stop = false;
        while !stop {
            let sync = self.sync().await?;

            if let Some(torrent) = sync.torrents().as_ref().and_then(|t| t.get(hash)) {
                if torrent.progress().as_ref() == Some(1.0 as f64).as_ref() {
                    return Ok(true);
                }
                let eta = match torrent.eta().as_ref().map(|eta| eta.get().to_owned()) {
                    Some(eta) => eta,
                    _ => DEFAULT_TIMEOUT,
                };

                if eta == QBITTORRENT_INFINITE {
                    return Ok(false);
                }

                println!(
                    "ETA: {} min, progress {}",
                    eta / 60,
                    torrent.progress().unwrap()
                );
                let adjusted_eta = eta.min(MAX_TIMEOUT).max(MIN_TIMEOUT);

                println!("{:?}", adjusted_eta);
                sleep(Duration::from_secs(adjusted_eta)).await;
            } else {
                stop = true;
            }
        }

        Err(Error::new(
            crate::ErrorKind::TorrentNotFound,
            "Torrent not found",
        ))
    }
}
