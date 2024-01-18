use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};
use std::io::Error as IoError;
use torrent_search_client::InvalidOptionError;

#[derive(Debug, Responder)]
pub enum HttpErrorKind {
    #[response(status = 400)]
    InvalidParam(String),
    MissingQuery(String),
    QbittorrentError(String),
    IoError(IoError),
    InvalidMagnet(String),
    TorrentIsFile(String),
    MovieFileNotFound(String),
    TorrentNotFound(String),
    MovieInfoError(String),
}

impl HttpErrorKind {
    pub fn param(param: String) -> Self {
        Self::InvalidParam(format!("Incorrect param: {}", param))
    }
    pub fn missing_query() -> Self {
        Self::MissingQuery("At least `imdb` or `query` must be defined.".into())
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
            HttpErrorKind::QbittorrentError(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "QBITTORRENT_ERROR",
                }),
            ),
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
            HttpErrorKind::TorrentIsFile(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "TORRENT_IS_FILE",
                }),
            ),
            HttpErrorKind::TorrentNotFound(error) => FieldError::new(
                error,
                graphql_value!({
                    "type": "TORRENT_NOT_FOUND",
                }),
            ),
            HttpErrorKind::MovieInfoError(error) => FieldError::new(
                error,
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
        }
    }
}

impl From<qbittorrent_api::Error> for HttpErrorKind {
    fn from(err: qbittorrent_api::Error) -> Self {
        Self::QbittorrentError(err.kind().to_string())
    }
}

impl From<IoError> for HttpErrorKind {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}

impl From<movie_info::Error> for HttpErrorKind {
    fn from(value: movie_info::Error) -> Self {
        Self::MovieInfoError(value.to_string())
    }
}
