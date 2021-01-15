use std::io::Error as IOError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CLIError {
    #[error("IOError: {0}")]
    IOError(IOError),
    #[error("ResolveError: {0}")]
    ResolveError(String),
    #[error("ParseError: {0}")]
    ParseError(String),
}

impl From<IOError> for CLIError {
    fn from(io_error: IOError) -> Self {
        CLIError::IOError(io_error)
    }
}
