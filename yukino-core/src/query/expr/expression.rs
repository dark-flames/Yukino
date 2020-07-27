use crate::query::expr::logical::LogicalExpression;
use crate::query::expr::mathematical::MathematicalExpression;
use crate::query::{Function, IdentExpression, SubqueryExpression, Value};
use syn::parse::{Parse, ParseBuffer};
use syn::Error;

pub enum Expression {
    MathematicalExpr(MathematicalExpression),
    LogicalExpr(LogicalExpression),
    SubqueryExpr(SubqueryExpression),
    IdentExpr(IdentExpression),
    Function(Function),
    Value(Value),
}

impl Parse for Expression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}

#[allow(dead_code)]
impl Expression {
    fn parse_item<'a>(_input: &'a ParseBuffer<'a>) -> Result<Expression, Error> {
        unimplemented!()
    }
}