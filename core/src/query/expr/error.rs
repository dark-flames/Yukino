use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExprParseError {
    #[error("Cannot parse token stream into DatabaseIdent")]
    CannotParseIntoIdent,
    #[error("Cannot parse token stream into Literal")]
    CannotParseIntoLit,
    #[error("Cannot parse token stream into Function")]
    CannotParseIntoFunction,
    #[error("Cannot parse \"{0}\" into string")]
    CannotParseFloat(String),
}
