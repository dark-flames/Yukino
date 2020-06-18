use proc_macro2::{TokenStream, Ident};
use yui::AttributeStructure;
use std::error::Error;
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
    ) -> AttributeError {
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