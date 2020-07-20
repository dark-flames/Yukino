use crate::query::Expression;
use syn::parse::{Parse, ParseBuffer};
use syn::Error;

pub enum Function {
    Average(Box<Expression>),
    Max(Box<Expression>),
    Min(Box<Expression>),
    Count(Box<Expression>),
    Distinct(Box<Expression>), // todo move to other group?
    Abs(Box<Expression>),
    Contact(Vec<Expression>),
}

impl Parse for Function {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}