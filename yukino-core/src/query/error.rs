use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Fail to get definitions of entity: `{0}`")]
    UnknownEntity(&'static str),
}
