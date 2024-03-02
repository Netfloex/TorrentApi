use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};
use std::io::Error as IoError;
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

impl From<InvalidOptionError> for HttpErrorKind {
    fn from(err: InvalidOptionError) -> Self {
        Self::param(err.option().to_string())
    }
}

impl<S: ScalarValue> IntoFieldError<S> for HttpErrorKind {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            HttpErrorKind::InvalidParam(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "INCORRECT_PARAM"
                }),
            ),
            HttpErrorKind::MissingQuery(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "MISSING_QUERY"
                }),
            ),
            HttpErrorKind::QbittorrentError(error) => {
                let kind = error.kind().to_string();
                let message = error.message();

                FieldError::new(
                    error.to_string(),
                    graphql_value!({
                        "type": "QBITTORRENT_ERROR",
                        "kind": kind,
                        "message": message,
                    }),
                )
            }
            HttpErrorKind::IoError(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "IO_ERROR",
                }),
            ),
            HttpErrorKind::InvalidMagnet(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "INVALID_MAGNET",
                }),
            ),
            HttpErrorKind::TorrentNotFound(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "TORRENT_NOT_FOUND",
                }),
            ),
            HttpErrorKind::MovieInfoError(error) => FieldError::new(
                error.to_string(),
                graphql_value!({
                    "type": "MOVIE_INFO_ERROR",
                }),
            ),
            HttpErrorKind::MovieFileNotFound(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "MOVIE_FILE_NOT_FOUND",
                }),
            ),
            HttpErrorKind::ImdbNotFound(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "IMDB_NOT_FOUND",
                }),
            ),
        }
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
