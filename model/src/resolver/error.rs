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
    #[error("FieldResolverNotFound: FieldResolver for {0} in {1} is not found")]
    FieldResolverNotFound(EntityPath, FieldName),
}

impl ResolveError {
    pub fn into_syn_error<T: ToTokens>(self, tokens: T) -> Error {
        Error::new_spanned(tokens, self)
    }
}
