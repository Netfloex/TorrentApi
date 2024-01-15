use crate::{
    graphql::ContextPointer,
    http_error::HttpErrorKind,
    utils::{get_tmdb::get_tmdb, import_movie::import_movie, movie_info::MovieInfo},
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tokio::time::sleep;

// const QBITTORRENT_INFINITE: u64 = 8640000;
const MAX_TIMEOUT_ACTIVE: u64 = 60;
const TIMEOUT_INACTIVE: u64 = 300;
const MIN_TIMEOUT: u64 = 1;

pub async fn movie_tracking(context: ContextPointer) -> Result<(), HttpErrorKind> {
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
        let mut min_eta = MAX_TIMEOUT_ACTIVE;
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
            let mut ctx = context.lock().await;
            let category = ctx.config().qbittorrent().category().to_owned();
            let movies_path = ctx.config().movies_path().to_owned();
            let remote_download_path = ctx.config().remote_download_path().to_owned();
            let local_download_path = ctx.config().local_download_path().to_owned();

            let qb = ctx.qbittorrent_client_mut();

            println!("Checking for torrents to import");
            let sync = qb.sync().await?;

            if let Some(torrents) = sync.torrents().clone() {
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

                            min_eta = min_eta.min(eta).min(MAX_TIMEOUT_ACTIVE).max(MIN_TIMEOUT);

                            println!(
                                "{}: Progress: {:.2}%, ETA: {} min, State: {:?}",
                                name,
                                (progress * 100.0).round(),
                                eta / 60,
                                state
                            );
                        } else {
                            if let Some(tmdb) = get_tmdb(name) {
                                let movie = MovieInfo::from_tmdb(tmdb).await;
                                let movie_name = movie.format();

                                println!("Importing \"{}\" as \"{}\"", name, movie_name);

                                let remote_path = torrent
                                    .content_path()
                                    .as_ref()
                                    .expect("Content path should be available at sync");

                                let local_path = remote_path
                                    .replace(&remote_download_path, &local_download_path);
                                let local_path = PathBuf::from(local_path);

                                let dest_folder = movies_path.join(movie_name);

                                import_movie(local_path, dest_folder).await?;

                                qb.set_category(hash.to_owned(), "".to_string()).await?;
                            } else {
                                println!("No TMDB id found for {}", name);
                            }
                        }
                    }
                }

                if watching_torrents == 0 {
                    println!("No torrents to track");
                    ctx.disable_movie_tracking().await;
                    continue;
                } else if active_torrents == 0 {
                    min_eta = TIMEOUT_INACTIVE;
                    println!("No active torrents, waiting {}s", min_eta)
                } else {
                    println!(
                        "Watching {}/{} torrents",
                        active_torrents, watching_torrents
                    )
                }
            } else {
                println!("No torrents in sync");
                ctx.disable_movie_tracking().await;
                continue;
            }
        }

        println!("Waiting: {}s", min_eta);
        sleep(Duration::from_secs(min_eta)).await;
    }
}
