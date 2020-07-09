use std::error::Error;
use syn::export::ToTokens;
use yui::AttributeStructure;
use proc_macro2::{TokenStream, Ident};
use syn::export::fmt::{Display, Formatter, Result};


pub trait CompileError: Error + Display {
    fn get_message(&self) -> String;

    fn to_compile_error(&self) -> TokenStream {
        let message = self.get_message();

        quote::quote! {
            compile_error!(#message);
        }
    }
}

#[derive(Debug)]
pub struct AttributeError (String);

impl AttributeError {
    #[allow(dead_code)]
    pub fn new<T: AttributeStructure, D: Display>(
        ident: &Ident,
        message: &D
    ) -> Self {
        AttributeError(format!(
            "Attribute Error: Error('{}') occurred in attribute '{}' on '{}'",
            message,
            T::get_path(),
            ident
        ))
    }
}

impl Display for AttributeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for AttributeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl CompileError for AttributeError {
    fn get_message(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug)]
pub struct TypeError (String);

impl TypeError {
    #[allow(dead_code)]
    pub fn new<D: Display + ?Sized>(
        value_type: &dyn ToTokens,
        message: &D
    ) -> Self {
        TypeError(format!(
            "Type Error: Error('{}') occurred in type: {}",
            message,
            value_type.to_token_stream()
        ))
    }
}

impl Display for TypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for TypeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl CompileError for TypeError {
    fn get_message(&self) -> String {
        self.0.clone()
    }
}

