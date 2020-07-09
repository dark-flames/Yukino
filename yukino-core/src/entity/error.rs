use crate::error::RuntimeError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct ParseError(String);

#[allow(dead_code)]
impl ParseError {
    pub fn new<T: Display + ?Sized>(message: &T) -> Self {
        ParseError(message.to_string())
    }
}

impl RuntimeError for ParseError {
    fn get_message(&self) -> String {
        self.0.clone()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
