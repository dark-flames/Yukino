use crate::query::syntax::expression::Expression;
use crate::types::DatabaseType;
use crate::query::syntax::error::SyntaxError;


#[derive(Copy, Clone)]
pub struct FunctionDefinition {
    pub ident: &'static str,
    pub ty_interpreter: &'static dyn FnOnce(Vec<Expression>) -> Result<DatabaseType, SyntaxError>
}

#[allow(dead_code)]
pub struct FunctionCall {
    function: FunctionDefinition,
    parameters: Vec<Expression>
}