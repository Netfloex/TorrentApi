use std::fmt;

use surf::StatusCode;

#[derive(Debug)]
pub enum ErrorKind {
    HttpRequestError(surf::Error),
    IncorrectLogin,
    TorrentAddError,
    BadParameters(String),
    RequestError,
}
impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            ErrorKind::HttpRequestError(error) => format!("RequestError: {}", error),
            ErrorKind::IncorrectLogin => "IncorrectLogin".into(),
            ErrorKind::TorrentAddError => "TorrentAddError".into(),
            ErrorKind::BadParameters(param) => format!("Bad Parameter: {}", param),
            ErrorKind::RequestError => "RequestError".into(),
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
