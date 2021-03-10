use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::location::Location;
use crate::query::ast::node::{Node, QueryPair};
use crate::query::grammar::Rule;

pub struct Boolean {
    pub value: bool,
    pub location: Location,
}

impl Node for Boolean {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::bool => {
                let value = match pair.into_inner().next().unwrap().as_rule() {
                    Rule::bool_true => true,
                    Rule::bool_false => false,
                    _ => return Err(location.error(SyntaxError::UnexpectedPair("bool"))),
                };

                Ok(Boolean { value, location })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("bool"))),
        }
    }
}

#[test]
pub fn test_bool() {
    use crate::pest::Parser;
    use crate::query::grammar::Grammar;

    let result1 = Grammar::parse(Rule::bool, "true").unwrap().next().unwrap();
    let result2 = Grammar::parse(Rule::bool, "false").unwrap().next().unwrap();

    let lit1 = Boolean::from_pair(result1).unwrap();
    let lit2 = Boolean::from_pair(result2).unwrap();
    assert!(lit1.value);
    assert!(!lit2.value);
}

pub struct Integer {
    pub value: i128,
    pub location: Location,
}
pub enum Literal {
    Boolean(Boolean),
    Integer,
    Float,
    String,
    Char,
    External,
    Null,
}
