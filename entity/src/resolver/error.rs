use crate::resolver::{EntityName, FieldName};
use quote::ToTokens;
use std::io::Error as IOError;
use syn::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("UnsupportedEntityStructure: Enum or Union structure are not supported for Entity")]
    UnsupportedEntityStructure,
    #[error("UnsupportedEntityStructType: Field of entity struct must be named field")]
    UnsupportedEntityStructType,
    #[error("EntityResolverNotFound: EntityResolver for {0} is not found")]
    EntityResolverNotFound(EntityName),
    #[error("EntityResolverIsNotFinished: EntityResolver for {0} is not finished")]
    EntityResolverIsNotFinished(EntityName),
    #[error("FieldResolverNotFound: FieldResolver for {1} in {0} is not found")]
    FieldResolverNotFound(EntityName, FieldName),
    #[error("FailToAssembleField: Fail to assemble field {1} in {0} is not found")]
    FailToAssembleField(EntityName, FieldName),
    #[error("UnfinishedFieldCanNotAssembleToEntity: Unfinished field({1} in {0}) can not assemble to entity resolver")]
    UnfinishedFieldCanNotAssembleToEntity(EntityName, FieldName),
    #[error("FieldResolverIsNotFinished: Field resolver for {1} in {0} is not finished")]
    FieldResolverIsNotFinished(EntityName, FieldName),
    #[error("FieldResolverStillSeed: Unexpect resolver status: Seed")]
    FieldResolverStillSeed,
    #[error("NoSuitableResolverSeedsFound: No suitable resolver seeds found for {1} in {0}")]
    NoSuitableResolverSeedsFound(EntityName, FieldName),
    #[error("GlobInPathIsNotSupported: Glob in path({0}) is not supported")]
    GlobInPathIsNotSupported(String),
    #[error("GlobInPathIsNotSupported: Schema file only support `struct` and `use` block")]
    UnsupportedSyntaxBlock,
    #[error("GenericIsNotSupported: Generic is not supported on entity struct: {0}")]
    GenericIsNotSupported(EntityName),
    #[error("EntityVisibilityMustBePublic: Visibility of entity({0}) must be public")]
    EntityVisibilityMustBePublic(EntityName),
    #[error("EntityVisibilityMustBePrivate: Visibility of field({1} in {0}) must be private")]
    FieldVisibilityMustBePrivate(EntityName, FieldName),
    #[error("IOError: {0}")]
    IOError(IOError),
    #[error("ParseError: {0}")]
    ParseError(String),
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
    UnexpectedDatabaseValueType(EntityName, FieldName),
    #[error("UnsupportedFieldType: \"{0}\" type is not supported by {1}")]
    UnsupportedFieldType(String, &'static str),
    #[error("DatabaseValueConvertError: Error({0}) occur while converting field({2} in {1})")]
    DatabaseValueConvertError(String, EntityName, FieldName),
}

impl From<DataConvertError> for ResolveError {
    fn from(e: DataConvertError) -> Self {
        ResolveError::Others(e.to_string())
    }
}
