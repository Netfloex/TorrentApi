use torrent_search_client::InvalidOptionError;

#[derive(Debug, Responder)]
pub enum HttpErrorKind {
    #[response(status = 400)]
    InvalidParam(String),
}

impl From<InvalidOptionError> for HttpErrorKind {
    fn from(err: InvalidOptionError) -> Self {
        HttpErrorKind::InvalidParam(format!("Incorrect param: {}", err.option().to_string()))
    }
}
