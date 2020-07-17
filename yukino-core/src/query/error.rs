use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Fail to get definitions of entity: `{0}`.")]
    UnknownEntity(&'static str),
    #[error("Table `{0}` is already existed.")]
    ExistedTableName(String),
    #[error("Alias of table`{0}` conflict")]
    ConflictAlias(String),
}
