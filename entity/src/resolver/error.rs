use crate::resolver::{EntityPath, FieldName};
use quote::ToTokens;
use syn::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("UnsupportedEntityStructure: Enum or Union structure are not supported for Entity")]
    UnsupportedEntityStructure,
    #[error("UnsupportedEntityStructType: Field of entity struct must be named field")]
    UnsupportedEntityStructType,
    #[error("EntityResolverNotFound: EntityResolver for {0} is not found")]
    EntityResolverNotFound(EntityPath),
    #[error("EntityResolverIsNotFinished: EntityResolver for {0} is not finished")]
    EntityResolverIsNotFinished(EntityPath),
    #[error("FieldResolverNotFound: FieldResolver for {1} in {0} is not found")]
    FieldResolverNotFound(EntityPath, FieldName),
    #[error("FailToAssembleField: Fail to assemble field {1} in {0} is not found")]
    FailToAssembleField(EntityPath, FieldName),
    #[error("UnfinishedFieldCanNotAssembleToEntity: Unfinished field({1} in {0}) can not assemble to entity resolver")]
    UnfinishedFieldCanNotAssembleToEntity(EntityPath, FieldName),
    #[error("FieldResolverIsNotFinished: Field resolver for {1} in {0} is not finished")]
    FieldResolverIsNotFinished(EntityPath, FieldName),
    #[error("FieldResolverStillSeed: Unexpect resolver status: Seed")]
    FieldResolverStillSeed,
    #[error("NoSuitableResolverSeedsFound: No suitable resolver seeds found for {1} in {0}")]
    NoSuitableResolverSeedsFound(EntityPath, FieldName),
    #[error("GlobInPathIsNotSupported: Glob in path({0}) is not supported")]
    GlobInPathIsNotSupported(String),
    #[error("{0}")]
    Others(String),
}

impl ResolveError {
    pub fn into_syn_error<T: ToTokens>(self, tokens: T) -> Error {
        Error::new_spanned(tokens, self)
    }
}

#[derive(Error, Debug)]
pub enum DataConvertError {
    #[error("UnexpectedDatabaseValueType: Unexpected type of DatabaseValue on field({1} in {0})")]
    UnexpectedDatabaseValueType(EntityPath, FieldName),
    #[error("UnsupportedFieldType: \"{0}\" type is not supported by {1}")]
    UnsupportedFieldType(String, &'static str),
    #[error("DatabaseValueConvertError: Error({0}) occur while converting field({2} in {1})")]
    DatabaseValueConvertError(String, EntityPath, FieldName),
}

impl From<DataConvertError> for ResolveError {
    fn from(e: DataConvertError) -> Self {
        ResolveError::Others(e.to_string())
    }
}
