use crate::{graphql::ContextPointer, http_error::HttpErrorKind};
use qbittorrent_api::AddTorrentOptions;

pub async fn track_movie(context: &ContextPointer, url: String) -> Result<(), HttpErrorKind> {
    let mut ctx = context.lock().await;

    let category = ctx.config().qbittorrent().category().to_owned();
    let qb = ctx.qbittorrent_client_mut();

    let options = AddTorrentOptions::default().category(category);

    qb.add_torrent(url, options).await?;

    ctx.enable_movie_tracking();

    Ok(())
}
