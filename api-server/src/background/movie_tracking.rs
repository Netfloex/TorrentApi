use crate::{
    config::Config,
    context::ContextPointer,
    http_error::HttpErrorKind,
    utils::{get_tmdb::get_tmdb, import_movie::import_movie},
};
use qbittorrent_api::PartialTorrent;
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Duration};
use tokio::time::sleep;

pub async fn movie_tracking(context: ContextPointer) -> Result<(), HttpErrorKind> {
    let config: Config;

    {
        let ctx = context.lock().await;
        config = ctx.config().clone();
    }

    if config.disable_movie_tracking().to_owned() {
        println!("Movie progress check is disabled");
        return Ok(());
    }

    println!("Starting background movie progress tracking");

    let max_timeout_active = config.movie_tracking_max_timeout_active().to_owned();
    let timeout_inactive = config.movie_tracking_timeout_inactive().to_owned();
    let min_timeout = config.movie_tracking_min_timeout().to_owned();

    loop {
        let mut min_eta = max_timeout_active;
        while !context
            .lock()
            .await
            .movie_tracking_enabled()
            .lock()
            .await
            .to_owned()
        {
            println!("Progress tracking (temporarily) disabled");
            let ntfy = Arc::clone(&context.lock().await.movie_tracking_ntfy());
            ntfy.notified().await;
        }

        {
            let torrents: HashMap<String, PartialTorrent>;

            let category = config.qbittorrent().category().to_owned();
            let movies_path = config.movies_path().to_owned();
            let remote_download_path = config.remote_download_path().to_owned();
            let local_download_path = config.local_download_path().to_owned();

            {
                let mut ctx = context.lock().await;

                let qb = ctx.qbittorrent_client_mut();

                println!("Checking for torrents to import");
                let sync = qb.sync().await?;
                torrents = sync.torrents().clone().unwrap_or_default();
            }

            let mut watching_torrents = 0;
            let mut active_torrents = 0;

            for (hash, torrent) in torrents {
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
                        if state.is_active() {
                            active_torrents += 1;
                        }

                        min_eta = min_eta.min(eta).min(max_timeout_active).max(min_timeout);

                        println!(
                            "{}: Progress: {:.2}%, ETA: {} min, State: {:?}",
                            name,
                            (progress * 100.0).round(),
                            eta / 60,
                            state
                        );
                    } else {
                        if let Some(tmdb) = get_tmdb(name) {
                            let movie = context
                                .lock()
                                .await
                                .movie_info_client()
                                .from_tmdb(tmdb)
                                .await?;
                            let movie_name = movie.format();

                            println!("Importing \"{}\" as \"{}\"", name, movie_name);

                            let remote_path = torrent
                                .content_path()
                                .as_ref()
                                .expect("Content path should be available at sync");

                            let local_path =
                                remote_path.replace(&remote_download_path, &local_download_path);
                            let local_path = PathBuf::from(local_path);

                            let dest_folder = movies_path.join(movie_name);

                            import_movie(local_path, dest_folder).await?;

                            context
                                .lock()
                                .await
                                .qbittorrent_client()
                                .set_category(hash.to_owned(), "".to_string())
                                .await?;
                        } else {
                            println!("No TMDB id found for {}", name);
                        }
                    }
                }
            }

            if watching_torrents == 0 {
                println!("No torrents to track");
                context.lock().await.disable_movie_tracking().await;
                continue;
            } else if active_torrents == 0 {
                min_eta = timeout_inactive;
                println!("No active torrents, waiting {}s", min_eta)
            } else {
                println!(
                    "Watching {}/{} torrents",
                    active_torrents, watching_torrents
                )
            }
        }

        println!("Waiting: {}s", min_eta);
        sleep(Duration::from_secs(min_eta)).await;
    }
}
