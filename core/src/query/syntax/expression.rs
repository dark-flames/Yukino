use crate::query::syntax::literal::Literal;
use crate::types::DatabaseType;
use crate::query::syntax::ident::DatabaseIdent;
use crate::query::syntax::unary::UnaryExpression;
use crate::query::syntax::binary::BinaryExpression;
use crate::query::syntax::function::FunctionCall;

pub enum TypeFlag {
    Resolved(DatabaseType),
    Unresolved
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