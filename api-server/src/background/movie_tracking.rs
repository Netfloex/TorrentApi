use crate::{graphql::ContextPointer, utils::get_tmdb::get_tmdb};
use std::time::Duration;
use tokio::time::sleep;

// const QBITTORRENT_INFINITE: u64 = 8640000;
const DEFAULT_TIMEOUT: u64 = 5;
const MAX_TIMEOUT: u64 = 60;
const MIN_TIMEOUT: u64 = 1;

pub async fn movie_tracking(context: ContextPointer) -> Result<(), qbittorrent_api::Error> {
    if context
        .lock()
        .await
        .config()
        .disable_movie_tracking()
        .to_owned()
    {
        println!("Movie progress check is disabled");
        return Ok(());
    }

    println!("Starting background movie progress tracking");

    loop {
        let mut min_eta = DEFAULT_TIMEOUT;
        if !context.lock().await.movie_tracking_enabled() {
            println!("Progress check (temporarily) disabled");
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        {
            let mut ctx = context.lock().await;
            let category = ctx.config().qbittorrent().category().to_owned();
            let qb = ctx.qbittorrent_client_mut();

            println!("Checking for torrents to import");
            let sync = qb.sync().await?;

            if let Some(torrents) = sync.torrents() {
                let mut watching_torrents = 0;

                torrents.values().for_each(|torrent| {
                    if torrent.category().as_ref() == Some(&category) {
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

                        let unknown = "Unknown".to_owned();
                        let name = torrent.name().as_ref().unwrap_or(&unknown);

                        if progress != 1.0 {
                            watching_torrents += 1;

                            min_eta = min_eta.min(eta).min(MAX_TIMEOUT).max(MIN_TIMEOUT);

                            println!(
                                "{}: Progress: {:.2}%, ETA: {} min, State: {:?}",
                                name,
                                (progress * 100.0).round(),
                                eta / 60,
                                state
                            );
                        } else {
                            if let Some(tmdb) = get_tmdb(name) {
                                println!("Importing {}", name);
                            } else {
                                println!("No TMDB id found for {}", name);
                            }
                        }
                    }
                });

                if watching_torrents == 0 {
                    println!("No torrents to import");
                    ctx.disable_movie_tracking();
                } else {
                    println!("Watching {} torrents", watching_torrents)
                }
            } else {
                println!("No torrents in sync");
                ctx.disable_movie_tracking();
            }
        }

        println!("Waiting: {}s", min_eta);
        sleep(Duration::from_secs(min_eta)).await;
    }
}
