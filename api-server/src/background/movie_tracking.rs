use crate::{
    graphql::ContextPointer,
    http_error::HttpErrorKind,
    utils::{get_tmdb::get_tmdb, import_movie::import_movie, movie_info::MovieInfo},
};
use std::{path::PathBuf, time::Duration};
use tokio::time::sleep;

// const QBITTORRENT_INFINITE: u64 = 8640000;
const DEFAULT_TIMEOUT: u64 = 5;
const MAX_TIMEOUT: u64 = 60;
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
        let mut min_eta = DEFAULT_TIMEOUT;
        if !context.lock().await.movie_tracking_enabled() {
            println!("Progress check (temporarily) disabled");
            sleep(Duration::from_secs(10)).await;
            continue;
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

            if let Some(torrents) = sync.torrents() {
                let mut watching_torrents = 0;

                for torrent in torrents.values() {
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
                            } else {
                                println!("No TMDB id found for {}", name);
                            }
                        }
                    }
                }

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
