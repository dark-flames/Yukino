use crate::resolver::EntityPath;
use quote::ToTokens;
use syn::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("UnsupportedEntityStructure: Enum or Union structure are not supported for Entity")]
    UnsupportedEntityStructure,
    #[error("UnsupportedEntityStructType: Field of entity struct must be named field")]
    UnsupportedEntityStructType,
    #[error("EntityResolverNotFound: EntityResolver of {0} is not found")]
    EntityResolverNotFound(EntityPath),
}

impl ResolveError {
    pub fn into_syn_error<T: ToTokens>(self, tokens: T) -> Error {
        Error::new_spanned(tokens, self)
    }
}
