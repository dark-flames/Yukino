use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Expect an alias here")]
    ExpectAnAlias,
    #[error("Order method must be \"asc\" or \"desc\"")]
    CannotParseIntoOrder,
}
