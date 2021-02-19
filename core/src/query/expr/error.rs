use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExprParseError {
    #[error("Cannot parse token stream into DatabaseIdent")]
    CannotParseIntoIdent,
}
