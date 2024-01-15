use surf::Error as SurfError;
#[derive(Debug)]
pub enum Error {
    RequestError(SurfError),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::RequestError(error) => format!("RequestError: {}", error),
        }
    }
}

impl From<SurfError> for Error {
    fn from(request_error: SurfError) -> Self {
        Self::RequestError(request_error)
    }
}
