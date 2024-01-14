use crate::{graphql::ContextPointer, http_error::HttpErrorKind};
use qbittorrent_api::AddTorrentOptions;
use utils::magnet::Magnet;

pub async fn track_movie(
    context: &ContextPointer,
    url: String,
    imdb: String,
) -> Result<(), HttpErrorKind> {
    let magnet = Magnet::from_url(&url).map_err(|err| HttpErrorKind::InvalidMagnet(err))?;
    let display_name = magnet.display_name().to_owned();

    let mut ctx = context.lock().await;

    let category = ctx.config().qbittorrent().category().to_owned();
    let qb = ctx.qbittorrent_client_mut();

    let options = AddTorrentOptions::default()
        .category(category)
        .rename(format!("{} ({})", display_name, imdb));

    qb.add_torrent(url, options).await?;

    ctx.enable_movie_tracking();

    Ok(())
}
