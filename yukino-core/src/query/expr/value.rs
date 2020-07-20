use proc_macro2::Ident;
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Lit};

pub enum Value {
    Lit(Lit),
    ExternalValue(Ident),
}

impl Parse for Value {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}
