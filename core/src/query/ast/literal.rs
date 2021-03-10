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

impl Node for Integer {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::int => {
                let inner = pair.as_str();
                let value = inner.parse().map_err(|_| {
                    location.error(SyntaxError::CannotParseIntoInteger(inner.to_string()))
                })?;

                Ok(Integer { value, location })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("int"))),
        }
    }
}

#[test]
pub fn test_integer() {
    use crate::pest::Parser;
    use crate::query::grammar::Grammar;

    let result1 = Grammar::parse(Rule::int, "114514").unwrap().next().unwrap();
    let result2 = Grammar::parse(Rule::int, "-114514")
        .unwrap()
        .next()
        .unwrap();

    let lit1 = Integer::from_pair(result1).unwrap();
    let lit2 = Integer::from_pair(result2).unwrap();
    assert_eq!(lit1.value, 114514);
    assert_eq!(lit2.value, -114514);
}

#[allow(dead_code)]
pub struct Float {
    value: f64,
    location: Location,
}

impl Node for Float {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::float => {
                let inner = pair.as_str();
                let value = inner.parse().map_err(|_| {
                    location.error(SyntaxError::CannotParseIntoFloat(inner.to_string()))
                })?;

                Ok(Float { value, location })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("float"))),
        }
    }
}

#[test]
pub fn test_float() {
    use crate::pest::Parser;
    use crate::query::grammar::Grammar;
    use float_eq::assert_float_eq;

    let result1 = Grammar::parse(Rule::float, "114.514")
        .unwrap()
        .next()
        .unwrap();
    let result2 = Grammar::parse(Rule::float, "-1e10")
        .unwrap()
        .next()
        .unwrap();

    let lit1 = Float::from_pair(result1).unwrap();
    let lit2 = Float::from_pair(result2).unwrap();
    assert_float_eq!(lit1.value, 114.514, ulps <= 4);
    assert_float_eq!(lit2.value, -1e10, ulps <= 4);
}

pub enum Literal {
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String,
    Char,
    External,
    Null,
}
