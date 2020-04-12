use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound,
    OperationFailed(Box<dyn error::Error>),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::NotFound => None,
            Error::OperationFailed(error) => Some(error.as_ref()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound => writeln!(f, "Could not find record matching email address",),
            Error::OperationFailed(error) => writeln!(f, "Storage operation failed: {}", error),
        }
    }
}
