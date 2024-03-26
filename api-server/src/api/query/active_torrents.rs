use super::super::get_context;
use crate::{
    models::http_error::HttpErrorKind, models::torrent_movie_info::TorrentMovieInfo,
    utils::get_tmdb::get_tmdb,
};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use movie_info::TmdbId;
use qbittorrent_api::{GetTorrentsParameters, Torrent};
use std::collections::{HashMap, HashSet};

#[derive(SimpleObject)]
#[graphql(complex)]
struct ActiveTorrentsResponse {
    torrents: Vec<Torrent>,
}

#[ComplexObject]
impl ActiveTorrentsResponse {
    async fn movie_info<'ctx>(
        &self,
        context: &Context<'ctx>,
    ) -> Result<Vec<TorrentMovieInfo>, HttpErrorKind> {
        let mut torrent_movie_info: HashMap<TmdbId, TorrentMovieInfo> = HashMap::new();

        let tmdb_ids: HashSet<TmdbId> = self
            .torrents
            .iter()
            .filter_map(|torrent| get_tmdb(torrent.get_name()))
            .collect();

        let movie_info = get_context(context)
            .movie_info_client()
            .bulk(&tmdb_ids)
            .await?;

        movie_info.iter().for_each(|info| {
            let torrents = self.torrents.iter().filter_map(|torrent| {
                get_tmdb(torrent.get_name()).as_ref().and_then(|tmdb| {
                    if info.get_tmdb_id() == tmdb {
                        Some(torrent)
                    } else {
                        None
                    }
                })
            });

            torrents.for_each(|torrent| {
                if let Some(tmdb) = get_tmdb(torrent.get_name()) {
                    if let Some(info) = torrent_movie_info.get_mut(&tmdb) {
                        info.add_torrent(torrent.get_hash().to_owned());
                    } else {
                        let mut info = TorrentMovieInfo::from(info);

                        info.add_torrent(torrent.get_hash().to_owned());

                        torrent_movie_info.insert(tmdb, info);
                    }
                }
            })
        });

        Ok(torrent_movie_info.into_values().collect())
    }
}

#[derive(Default)]
pub struct ActiveTorrentsQuery;

#[Object]
impl ActiveTorrentsQuery {
    async fn active_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        #[graphql(default)] params: GetTorrentsParameters,
    ) -> Result<ActiveTorrentsResponse, HttpErrorKind> {
        let ctx = get_context(context);

        let torrents = ctx.qbittorrent_client().torrents(&params).await?;

        Ok(ActiveTorrentsResponse { torrents })
    }
}
