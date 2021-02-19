use crate::query::expr::expression::Expression;

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionCall {
    ident: String,
    parameters: Vec<Expression>,
}
