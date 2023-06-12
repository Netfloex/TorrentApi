use std::{error, fmt};

#[derive(Debug)]
pub enum ErrorKind {
    HttpRequestError(reqwest::Error),
    HttpRequestMiddlewareError(reqwest_middleware::Error),
    ParsingError(serde_json::Error),
    ScrapingError(),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new<S: Into<String>>(kind: ErrorKind, message: S) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl From<serde_json::Error> for Error {
    fn from(serde_error: serde_json::Error) -> Self {
        Self::new(ErrorKind::ParsingError(serde_error), "Parsing Error")
    }
}

impl From<reqwest::Error> for Error {
    fn from(request_error: reqwest::Error) -> Self {
        Self::new(ErrorKind::HttpRequestError(request_error), "Request Error")
    }
}

impl From<reqwest_middleware::Error> for Error {
    fn from(request_error: reqwest_middleware::Error) -> Self {
        Self::new(
            ErrorKind::HttpRequestMiddlewareError(request_error),
            "Request Error",
        )
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.kind() {
            ErrorKind::HttpRequestError(e) => e.source(),
            ErrorKind::HttpRequestMiddlewareError(e) => e.source(),
            ErrorKind::ParsingError(e) => e.source(),
            ErrorKind::ScrapingError() => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
