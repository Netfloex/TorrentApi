use crate::{
    models::{context::ContextPointer, http_error::HttpErrorKind},
    utils::{get_tmdb::get_tmdb, import_movie::import_movie},
};
use filenamify::filenamify;
use log::{debug, info, warn};
use std::{path::PathBuf, time::Duration};
use tokio::time::sleep;

pub async fn movie_tracking(context: ContextPointer) -> Result<(), HttpErrorKind> {
    let config = context.config();

    if *config.disable_movie_tracking() {
        info!("Movie progress check is disabled");
        return Ok(());
    }

    info!("Starting background movie progress tracking");

    let max_timeout_active = config.movie_tracking_max_timeout_active();
    let timeout_inactive = config.movie_tracking_timeout_inactive();
    let min_timeout = config.movie_tracking_min_timeout();

    let qb = context.qbittorrent_client();

    qb.ensure_category(config.qbittorrent().category(), "")
        .await?;

    loop {
        let mut min_eta = *max_timeout_active;
        while !*context.movie_tracking_enabled().lock().await {
            info!("Progress tracking (temporarily) disabled");

            context.movie_tracking_ntfy().notified().await;
        }

        let category = config.qbittorrent().category();
        let movies_path = config.movies_path();
        let remote_download_path = config.remote_download_path();
        let local_download_path = config.local_download_path();

        debug!("Checking for torrents to import");

        let torrents = qb.torrents_sync().await?;

        let mut watching_torrents = 0;
        let mut active_torrents = 0;

        for torrent in torrents {
            if torrent.get_category() == category {
                let progress = torrent.get_progress();
                let eta = torrent.get_eta();
                let state = torrent.get_state();

                let name = torrent.get_name();

                if progress != &1.0 {
                    watching_torrents += 1;
                    if state.is_active() {
                        active_torrents += 1;
                    }

                    min_eta = min_eta.min(*eta).min(*max_timeout_active).max(*min_timeout);

                    debug!(
                        "{}: Progress: {:.2}%, ETA: {} min, State: {:?}",
                        name,
                        (progress * 100.0).round(),
                        eta / 60,
                        state
                    );
                } else if let Some(tmdb) = get_tmdb(name) {
                    let movie = context.movie_info_client().from_tmdb(tmdb).await?;

                    if let Some(movie) = movie {
                        let movie_name = movie.format();

                        info!("Importing \"{}\" as \"{}\"", name, movie_name);

                        let remote_path = torrent.get_content_path();

                        let local_path =
                            remote_path.replace(remote_download_path, local_download_path);
                        let local_path = PathBuf::from(local_path);

                        let dest_folder = movies_path.join(filenamify(&movie_name));

                        import_movie(
                            &local_path,
                            &dest_folder,
                            *config.import_movie_max_depth(),
                            config.subtitle_language_map(),
                        )
                        .await?;

                        if *config.delete_torrent_after_import() {
                            qb.delete_torrent(
                                torrent.get_hash().to_owned(),
                                *config.delete_torrent_files(),
                            )
                            .await?;
                        } else {
                            qb.set_category(
                                torrent.get_hash().to_owned(),
                                config.category_after_import().to_owned(),
                            )
                            .await?;
                        }
                    } else {
                        warn!("No movie found for TMDB id: {}", tmdb);
                    }
                } else {
                    warn!("No TMDB id found for {}", name);
                }
            }
        }

        if watching_torrents == 0 {
            info!("No torrents to track");
            context.disable_movie_tracking().await;
            continue;
        } else if active_torrents == 0 {
            min_eta = *timeout_inactive;
            info!("No active torrents")
        } else {
            info!(
                "Watching {}/{} torrents",
                active_torrents, watching_torrents
            )
        }

        info!("Waiting: {}s", min_eta);
        sleep(Duration::from_secs(min_eta as u64)).await;
    }
}
