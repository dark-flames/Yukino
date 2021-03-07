use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Unexpected alias")]
    UnexpectedAlias,
    #[error("Order method must be \"asc\" or \"desc\"")]
    CannotParseIntoOrder,
    #[error("Unexpected entity name")]
    UnexpectedEntityName,
}
