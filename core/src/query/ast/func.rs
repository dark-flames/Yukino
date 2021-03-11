use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::expr::Expr;
use crate::query::ast::traits::{FromPair, Locatable, QueryPair};
use crate::query::ast::Location;
use crate::query::grammar::Rule;

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub ident: String,
    pub parameters: Vec<Expr>,
    pub location: Location,
}

impl PartialEq for FunctionCall {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.parameters == other.parameters
    }
}

impl Eq for FunctionCall {}

impl FromPair for FunctionCall {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();

        match pair.as_rule() {
            Rule::function_call => {
                let mut inner = pair.into_inner();

                let ident_pair = inner
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("ident")))?;

                let ident = match ident_pair.as_rule() {
                    Rule::ident => Ok(ident_pair.as_str().to_string()),
                    _ => Err(location.error(SyntaxError::UnexpectedPair("ident"))),
                }?;
                let mut parameters = vec![];

                for item in inner {
                    parameters.push(Expr::from_pair(item)?)
                }

                Ok(FunctionCall {
                    ident,
                    parameters,
                    location,
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("function_call"))),
        }
    }
}

impl Locatable for FunctionCall {
    fn location(&self) -> Location {
        self.location
    }
}
