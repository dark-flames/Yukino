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
    #[error("NotSuitableResolverSeedsFound: No suitable resolver seeds found for {1} in {0}")]
    NotSuitableResolverSeedsFound(EntityPath, FieldName),
}

impl ResolveError {
    pub fn into_syn_error<T: ToTokens>(self, tokens: T) -> Error {
        Error::new_spanned(tokens, self)
    }
}
