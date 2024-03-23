use std::{fmt::Display, io::Error as IoError};
use torrent_search_client::InvalidOptionError;

#[derive(Debug)]
pub enum HttpErrorKind {
    InvalidParam(String),
    MissingQuery(String),
    QbittorrentError(qbittorrent_api::Error),
    IoError(IoError),
    InvalidMagnet(String),
    MovieFileNotFound(String),
    TorrentNotFound(String),
    MovieInfoError(movie_info::Error),
    ImdbNotFound(String),
}

impl HttpErrorKind {
    pub fn param(param: String) -> Self {
        Self::InvalidParam(format!("Incorrect param: {}", param))
    }
    pub fn missing_query() -> Self {
        Self::MissingQuery("At least `imdb` or `query` must be defined.".into())
    }
    pub fn imdb_not_found(imdb: String) -> Self {
        Self::ImdbNotFound(format!("IMDB ID not found: {}", imdb))
    }
}

impl Display for HttpErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<InvalidOptionError> for HttpErrorKind {
    fn from(err: InvalidOptionError) -> Self {
        Self::param(err.option().to_string())
    }
}
impl From<qbittorrent_api::Error> for HttpErrorKind {
    fn from(err: qbittorrent_api::Error) -> Self {
        Self::QbittorrentError(err)
    }
}

impl From<IoError> for HttpErrorKind {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

impl From<movie_info::Error> for HttpErrorKind {
    fn from(err: movie_info::Error) -> Self {
        Self::MovieInfoError(err)
    }
}
