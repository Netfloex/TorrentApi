use torrent_search_client::InvalidOptionError;

#[derive(Debug, Responder)]
pub enum HttpErrorKind {
    #[response(status = 400)]
    InvalidParam(String),
}

impl HttpErrorKind {
    pub fn param(param: String) -> Self {
        Self::InvalidParam(format!("Incorrect param: {}", param))
    }
}

impl From<InvalidOptionError> for HttpErrorKind {
    fn from(err: InvalidOptionError) -> Self {
        Self::param(err.option().to_string())
    }
}
