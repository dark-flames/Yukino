use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Expect an alias here")]
    ExpectAnAlias,
}
