use crate::query::query_builder::SelectQueryBuilder;
use crate::query::Expression;
use syn::parse::{Parse, ParseBuffer};
use syn::Error;

pub enum SubqueryExpression {
    In(Box<Expression>, Box<SelectQueryBuilder>),
    Any(Box<SelectQueryBuilder>),
    Some(Box<SelectQueryBuilder>),
    ALL(Box<SelectQueryBuilder>),
    Exists(Box<SelectQueryBuilder>),
}

impl Parse for SubqueryExpression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}
