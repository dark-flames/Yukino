use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error as StdError;
use thiserror::Error as ErrorDerive;



#[derive(Debug)]
pub struct Error {
    msg: String,
    content: String,
    pos: usize
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error: {} at pos {} in `{}`", self.msg, self.pos, self.content)
    }
}

impl Error {
    pub fn head<E: Display, T: Display>(msg: E, content: T) -> Self {
        Error {
            msg: msg.to_string(),
            content: content.to_string(),
            pos: 0
        }
    }
}

#[derive(ErrorDerive, Debug)]
pub enum ParseError {
    #[error("Unknown token")]
    UnknownToken
}


