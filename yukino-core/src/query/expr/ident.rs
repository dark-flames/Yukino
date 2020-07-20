use syn::parse::{Parse, ParseBuffer};
use syn::Error;

#[allow(dead_code)]
pub struct IdentExpression {
    segments: Vec<String>,
}

impl Parse for IdentExpression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}
