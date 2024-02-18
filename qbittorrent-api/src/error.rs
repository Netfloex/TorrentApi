use std::fmt;
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

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            ErrorKind::HttpRequestError(error) => format!("RequestError: {}", error),
            ErrorKind::IncorrectLogin => "IncorrectLogin".into(),
            ErrorKind::TorrentAddError => "TorrentAddError".into(),
            ErrorKind::BadParameters(param) => format!("Bad Parameter: {}", param),
            ErrorKind::RequestError => "RequestError".into(),
            ErrorKind::TorrentNotFound => "TorrentNotFound".into(),
            ErrorKind::TorrentNotDownloading => "TorrentNotDownloading".into(),
            ErrorKind::CategoryDoesNotExist => "CategoryDoesNotExist".into(),
            ErrorKind::SerdeError(error) => format!("SerdeError: {}", error),
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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
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
