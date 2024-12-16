use std::fmt::Display;
use surf::StatusCode;

#[derive(Debug)]
pub enum ErrorKind {
    HttpRequestError(surf::Error),
    IncorrectLogin,
    TorrentAddError,

    BadParameters(String),
    RequestError,
    TorrentNotFound,
    TorrentNotDownloading,
    CategoryDoesNotExist,
    SerdeError(serde_json::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::HttpRequestError(error) => write!(f, "RequestError: {}", error),
            ErrorKind::IncorrectLogin => write!(f, "IncorrectLogin"),
            ErrorKind::TorrentAddError => write!(f, "TorrentAddError"),
            ErrorKind::BadParameters(param) => write!(f, "Bad Parameter: {}", param),
            ErrorKind::RequestError => write!(f, "RequestError"),
            ErrorKind::TorrentNotFound => write!(f, "TorrentNotFound"),
            ErrorKind::TorrentNotDownloading => write!(f, "TorrentNotDownloading"),
            ErrorKind::CategoryDoesNotExist => write!(f, "CategoryDoesNotExist"),
            ErrorKind::SerdeError(error) => write!(f, "SerdeError: {}", error),
        }
    }
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

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}>: {}", self.kind, self.message)
    }
}

impl From<surf::Error> for Error {
    fn from(request_error: surf::Error) -> Self {
        println!("{}", request_error);
        if request_error.status() == StatusCode::Unauthorized {
            return Self::new(ErrorKind::IncorrectLogin, "Incorrect login");
        }
        Self::new(ErrorKind::HttpRequestError(request_error), "Request Error")
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::new(ErrorKind::SerdeError(value), "Serde Error")
    }
}
