use torrent_search_client::Error;

#[derive(Debug, Responder)]
pub enum HttpErrorKind {
    #[response(status = 400)]
    InvalidParam(String),
}

impl From<Error> for HttpErrorKind {
    fn from(err: Error) -> Self {
        HttpErrorKind::InvalidParam(err.to_string())
    }
}
