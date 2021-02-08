use crate::query::expr::binary::BinaryExpression;
use crate::query::expr::function::FunctionCall;
use crate::query::expr::ident::DatabaseIdent;
use crate::query::expr::literal::Literal;
use crate::query::expr::unary::UnaryExpression;
use crate::types::DatabaseType;

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
