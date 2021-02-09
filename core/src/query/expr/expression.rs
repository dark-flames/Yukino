use crate::query::expr::binary::BinaryExpression;
use crate::query::expr::function::FunctionCall;
use crate::query::expr::ident::DatabaseIdent;
use crate::query::expr::literal::Literal;
use crate::query::expr::unary::UnaryExpression;
use crate::types::DatabaseType;
use syn::parse::{Parse, ParseBuffer};
use syn::Error;

#[derive(Eq, PartialEq, Debug)]
pub enum TypeFlag {
    Resolved(DatabaseType),
    Unresolved,
}

impl Default for TypeFlag {
    fn default() -> Self {
        TypeFlag::Unresolved
    }
}

pub enum Expression {
    Literal(Literal),
    Ident(DatabaseIdent),
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    Function(FunctionCall),
}

impl Parse for Expression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}
