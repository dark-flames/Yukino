use crate::query::expr::function::FunctionCall;
use crate::query::expr::ident::DatabaseIdent;
use crate::query::expr::literal::Literal;
use crate::query::expr::mathematical::ArithmeticOrLogicalExpression;
use crate::query::parse::{Error, Parse, ParseBuffer};

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Ident(DatabaseIdent),
    Function(FunctionCall),
    ArithmeticOrLogicalExpression(ArithmeticOrLogicalExpression),
}

impl Parse for Expression {
    fn parse(_buffer: &mut ParseBuffer) -> Result<Self, Error> {
        unimplemented!()
    }

    fn peek(_buffer: &ParseBuffer) -> bool {
        unimplemented!()
    }
}
