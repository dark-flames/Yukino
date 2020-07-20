use syn::{Lit, Error};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseBuffer};

pub enum Value {
    Lit(Lit),
    ExternalValue(Ident)
}

impl Parse for Value {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}