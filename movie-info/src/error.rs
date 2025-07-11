use std::fmt::Display;

use surf::Error as SurfError;
#[derive(Debug)]
pub enum Error {
    RequestError(SurfError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RequestError(error) => write!(f, "RequestError: {error}"),
        }
    }
}

impl From<SurfError> for Error {
    fn from(request_error: SurfError) -> Self {
        Self::RequestError(request_error)
    }
}
