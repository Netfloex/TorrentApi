use crate::{context::ContextPointer, http_error::HttpErrorKind};
use movie_info::TmdbId;
use qbittorrent_api::AddTorrentOptions;
use utils::magnet::Magnet;

pub async fn track_movie(
    ctx: &ContextPointer,
    url: String,
    tmdb: TmdbId,
) -> Result<(), HttpErrorKind> {
    let magnet = Magnet::from_url(&url).map_err(HttpErrorKind::InvalidMagnet)?;
    let display_name = magnet.display_name().to_owned();

    let category = ctx.config().qbittorrent().category().to_owned();
    let qb = ctx.qbittorrent_client();

    let options = AddTorrentOptions::default()
        .category(category)
        .rename(format!("{} ({})", display_name, tmdb));

    qb.add_torrent(url, options).await?;

    ctx.enable_movie_tracking().await;

    Ok(())
}
