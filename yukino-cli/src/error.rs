use std::error::Error;
use std::io::Error as IOError;
use std::fmt::{Formatter, Display, Result};


#[derive(Debug)]
pub struct FileError {
    filename: String,
    reason: String
}

impl FileError {
    pub fn new<D: Display + ?Sized>(filename: &D, io_error: IOError) -> Self {
        FileError {
            filename: filename.to_string(),
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

#[derive(Debug)]
pub struct ResolveError {
    filename: String,
    reason: String
}

impl ResolveError {
    pub fn new<D: Display + ?Sized>(filename: &str, message: &D) -> Self {
        ResolveError {
            filename: String::from(filename),
            reason: message.to_string()
        }
    }
}

impl Display for  ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Error occur while resolving file: {}, reason: {}", self.filename, self.reason)
    }
}

#[derive(Debug)]
pub struct OutputError {
    message: String
}

impl OutputError  {
    pub fn new<D: Display + ?Sized>(message: &D) -> Self {
        OutputError  {
            message: message.to_string()
        }
    }
}

impl Display for  OutputError  {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Error occur while writing implements: {}", self.message)
    }
}

impl Error for  OutputError  {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct YukinoCLIError {
    message: String
}

impl Display for YukinoCLIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Invalid command line arguments: {}", self.message)
    }
}

impl Error for YukinoCLIError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<ResolveError> for YukinoCLIError {
    fn from(e: ResolveError) -> Self {
        YukinoCLIError {
            message: e.to_string()
        }
    }
}

impl From<OutputError> for YukinoCLIError {
    fn from(e: OutputError) -> Self {
        YukinoCLIError {
            message: e.to_string()
        }
    }
}

impl From<FileError> for YukinoCLIError {
    fn from(e: FileError) -> Self {
        YukinoCLIError {
            message: e.to_string()
        }
    }
}

