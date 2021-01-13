use quote::ToTokens;
use std::error::Error;
use syn::Error as SynError;
use thiserror::Error;

pub trait RuntimeError: Error {}

pub trait CompileError: Error
where
    Self: Sized,
{
    fn into_syn_error<T: ToTokens>(self, tokens: T) -> SynError {
        SynError::new_spanned(tokens, self)
    }
}

#[derive(Error, Debug)]
pub enum DataConvertError {}

impl RuntimeError for DataConvertError {}
