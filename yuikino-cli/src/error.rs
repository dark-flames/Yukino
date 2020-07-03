use std::error::Error;
use std::fmt::{Formatter, Display, Result};
use std::io::Error as IOError;

#[derive(Debug)]
pub struct FileError {
    filename: String,
    reason: String
}

impl FileError {
    pub fn new(filename: &'static str, io_error: IOError) -> Self {
        FileError {
            filename: String::from(filename),
            reason: io_error.to_string()
        }
    }
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Fail to open file: {}, reason: {}", self.filename, self.reason)
    }
}

impl Error for FileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}