use crate::query::Expression;
use syn::parse::{Parse, ParseBuffer};
use syn::Error;

pub enum LogicalExpression {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

impl Parse for LogicalExpression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}