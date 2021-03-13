use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::location::Location;
use crate::query::ast::traits::{FromPair, Locatable, QueryPair};
use crate::query::grammar::Rule;
use float_eq::float_eq;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
    pub location: Location,
}

impl PartialEq for Boolean {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Boolean {}

impl FromPair for Boolean {
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

impl Locatable for Boolean {
    fn location(&self) -> Location {
        self.location
    }
}

#[test]
fn test_bool() {
    use crate::pest::Parser;
    use crate::query::grammar::Grammar;

    let result1 = Grammar::parse(Rule::bool, "true").unwrap().next().unwrap();
    let result2 = Grammar::parse(Rule::bool, "false").unwrap().next().unwrap();

    let lit1 = Boolean::from_pair(result1).unwrap();
    let lit2 = Boolean::from_pair(result2).unwrap();
    assert!(lit1.value);
    assert!(!lit2.value);
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i128,
    pub location: Location,
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Integer {}

impl FromPair for Integer {
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

impl Locatable for Integer {
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub struct Float {
    pub value: f64,
    pub location: Location,
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        float_eq!(self.value, other.value, ulps <= 4)
    }
}

impl Eq for Float {}

impl FromPair for Float {
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

impl Locatable for Float {
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Str {}

impl FromPair for Str {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::string => {
                let inner_pair = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("string_inner")))?;

                let value = match inner_pair.as_rule() {
                    Rule::string_inner => Ok(inner_pair.as_str().to_string()),
                    _ => Err(location.error(SyntaxError::UnexpectedPair("string_inner"))),
                }?;

                Ok(Str { value, location })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("string"))),
        }
    }
}

impl Locatable for Str {
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub struct ExternalValue {
    pub ident: String,
    pub location: Location,
}

impl PartialEq for ExternalValue {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Eq for ExternalValue {}

impl FromPair for ExternalValue {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::external_ident => {
                let inner_pair = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("ident")))?;

                let ident = match inner_pair.as_rule() {
                    Rule::ident => Ok(inner_pair.as_str().to_string()),
                    _ => Err(location.error(SyntaxError::UnexpectedPair("ident"))),
                }?;

                Ok(ExternalValue { ident, location })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("external_ident"))),
        }
    }
}

impl Locatable for ExternalValue {
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub struct Null {
    pub location: Location,
}

impl PartialEq for Null {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Null {}

impl FromPair for Null {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::null => Ok(Null { location }),
            _ => Err(location.error(SyntaxError::UnexpectedPair("null"))),
        }
    }
}

impl Locatable for Null {
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Literal {
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(Str),
    External(ExternalValue),
    Null(Null),
}

impl FromPair for Literal {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::literal => {
                let inner = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("literal")))?;

                Ok(match inner.as_rule() {
                    Rule::bool => Literal::Boolean(Boolean::from_pair(inner)?),
                    Rule::int => Literal::Integer(Integer::from_pair(inner)?),
                    Rule::float => Literal::Float(Float::from_pair(inner)?),
                    Rule::string => Literal::String(Str::from_pair(inner)?),
                    Rule::external_ident => Literal::External(ExternalValue::from_pair(inner)?),
                    Rule::null => Literal::Null(Null::from_pair(inner)?),
                    _ => return Err(location.error(SyntaxError::UnexpectedPair("literal"))),
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("literal"))),
        }
    }
}

impl Locatable for Literal {
    fn location(&self) -> Location {
        match self {
            Literal::Boolean(lit) => lit.location(),
            Literal::Integer(lit) => lit.location(),
            Literal::Float(lit) => lit.location(),
            Literal::String(lit) => lit.location(),
            Literal::External(lit) => lit.location(),
            Literal::Null(lit) => lit.location(),
        }
    }
}

#[test]
fn test_literal() {
    use crate::query::ast::helper::assert_parse_result;

    let location = Location::Pos(0);

    assert_parse_result(
        "true",
        Literal::Boolean(Boolean {
            value: true,
            location,
        }),
        Rule::literal,
    );
    assert_parse_result(
        "false",
        Literal::Boolean(Boolean {
            value: false,
            location,
        }),
        Rule::literal,
    );

    assert_parse_result(
        "114514",
        Literal::Integer(Integer {
            value: 114514,
            location,
        }),
        Rule::literal,
    );
    assert_parse_result(
        "-114514",
        Literal::Integer(Integer {
            value: -114514,
            location,
        }),
        Rule::literal,
    );

    assert_parse_result(
        "114.514",
        Literal::Float(Float {
            value: 114.514,
            location,
        }),
        Rule::literal,
    );
    assert_parse_result(
        "-1e10",
        Literal::Float(Float {
            value: -1e10,
            location,
        }),
        Rule::literal,
    );

    assert_parse_result(
        "\"\\n\\rtest\"",
        Literal::String(Str {
            value: "\\n\\rtest".to_string(),
            location,
        }),
        Rule::literal,
    );

    assert_parse_result(
        "$__external_value",
        Literal::External(ExternalValue {
            ident: "__external_value".to_string(),
            location,
        }),
        Rule::literal,
    );
    assert_parse_result(
        "$externa1_value",
        Literal::External(ExternalValue {
            ident: "externa1_value".to_string(),
            location,
        }),
        Rule::literal,
    );

    assert_parse_result("Null", Literal::Null(Null { location }), Rule::literal);
    assert_parse_result("null", Literal::Null(Null { location }), Rule::literal);
}
