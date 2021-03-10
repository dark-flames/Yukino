use crate::query::ast::Location;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

#[derive(Debug)]
pub struct SyntaxErrorWithPos {
    pub error: SyntaxError,
    pub location: Location,
}

impl StdError for SyntaxErrorWithPos {}

impl Display for SyntaxErrorWithPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error: {} at {:?}", self.error, self.location)
    }
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Expected pair to be a '{0}'")]
    UnexpectedPair(&'static str),
}
