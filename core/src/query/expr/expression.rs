use syn::parse::{ParseBuffer, Parse};
use crate::query::expr::literal::Literal;
use crate::query::expr::ident::DatabaseIdent;
use syn::Error;
use crate::query::expr::function::FunctionCall;
use crate::query::expr::mathematical::ArithmeticOrLogicalExpression;

#[allow(dead_code)]
pub enum Expression {
    Literal(Literal),
    Ident(DatabaseIdent),
    Function(FunctionCall),
    ArithmeticOrLogicalExpression(ArithmeticOrLogicalExpression),
}

impl Parse for Expression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}