use crate::query::expr::logical::LogicalExpression;
use crate::query::{SubqueryExpression, ComparativeExpression, IdentExpression, Function, Value};
use syn::parse::{Parse, ParseBuffer};
use syn::Error;
use crate::query::expr::mathematical::MathematicalExpression;

pub enum Expression {
    MathematicalExpr(MathematicalExpression),
    LogicalExpr(LogicalExpression),
    ComparativeExpr(ComparativeExpression),
    SubqueryExpr(SubqueryExpression),
    IdentExpr(IdentExpression),
    Function(Function),
    Value(Value)
}

impl Parse for Expression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}