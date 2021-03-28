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
    #[error("Cannot parse \"{0}\" into Integer")]
    CannotParseIntoInteger(String),
    #[error("Cannot parse \"{0}\" into Float")]
    CannotParseIntoFloat(String),
    #[error("Unexpected expr")]
    UnexpectedExpr,
    #[error("Conflict alias \"{0}\"")]
    ConflictAlias(String),
    #[error("Mismatched type: expected \"{0}\", found \"{1}\"")]
    TypeError(String, String),
    #[error("Conflict external value type assertion for \"#{0}\"")]
    ConflictValueAssertion(String),
    #[error("Unimplemented operation \"{0}\" for type \"{1}\"")]
    UnimplementedOperationForType(String, String),
    #[error("Literal out of range for \"{0}\"")]
    LitOverflow(String),
    #[error("Unknown alias \"{0}\"")]
    UnknownAlias(String),
    #[error("Unknown resolver name \"{0}\"")]
    UnknownResolverName(String),
    #[error("Cannot be wrapped into \"{0}\"")]
    CannotBeWrappedInto(String),
    #[error("Unable to infer a suitable type")]
    TypeInferError,
}
