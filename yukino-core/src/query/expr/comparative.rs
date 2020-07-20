use crate::query::Expression;
use syn::parse::{Parse, ParseBuffer};
use syn::Error;

pub enum ComparativeExpression {
    Eq(Box<Expression>, Box<Expression>),
    NotEq(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>),
}

impl Parse for ComparativeExpression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}
