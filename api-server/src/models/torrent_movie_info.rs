use async_graphql::SimpleObject;
use movie_info::{MovieInfo, TmdbId};
use std::collections::HashSet;

#[derive(PartialEq, Eq, SimpleObject)]
pub struct TorrentMovieInfo {
    title: String,
    year: u16,
    imdb: Option<String>,
    tmdb: TmdbId,
    runtime: u16,

    for_torrents: HashSet<String>,
}

impl TorrentMovieInfo {
    pub fn add_torrent(&mut self, hash: String) {
        self.for_torrents.insert(hash);
    }
}

impl From<&MovieInfo> for TorrentMovieInfo {
    fn from(info: &MovieInfo) -> Self {
        TorrentMovieInfo {
            title: info.get_title().to_owned(),
            year: info.get_year().to_owned(),
            imdb: info.get_imdb_id().to_owned(),
            tmdb: info.get_tmdb_id().to_owned(),
            runtime: info.get_runtime().to_owned(),
            for_torrents: HashSet::new(),
        }
    }
}
