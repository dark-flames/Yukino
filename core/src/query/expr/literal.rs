use crate::query::expr::error::ExprParseError;
use crate::query::parse::{Error, Lit, Parse, ParseBuffer, Peek, Symbol, Token};
use float_eq::float_eq;

#[derive(Debug)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    External(String),
}

impl Literal {
    pub fn from_lit(lit: Lit) -> Result<Self, ExprParseError> {
        Ok(match lit {
            Lit::Int(v) => Literal::Int(v as i64),
            Lit::Float(v) => {
                Literal::Float(v.parse().map_err(|_| ExprParseError::CannotParseFloat(v))?)
            }
            Lit::Bool(v) => Literal::Bool(v),
            Lit::String(s) => Literal::Str(s),
            Lit::Char(c) => Literal::Char(c),
            Lit::External(i) => Literal::External(i.to_string()),
        })
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Literal::Bool(x) => matches!(other, Literal::Bool(y) if x== y),
            Literal::Int(x) => matches!(other, Literal::Int(y) if x== y),
            Literal::Str(x) => matches!(other, Literal::Str(y) if x== y),
            Literal::Char(x) => matches!(other, Literal::Char(y) if x== y),
            Literal::External(x) => matches!(other, Literal::External(y) if x== y),
            Literal::Float(x) => matches!(other, Literal::Float(y) if float_eq!(x, y, ulps <= 4)),
        }
    }
}

impl Eq for Literal {}

impl Parse for Literal {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let before_cursor = buffer.cursor();
        let first = buffer.pop_token()?;

        if let Token::Lit(lit) = first {
            Ok(Literal::from_lit(lit).map_err(|e| buffer.error_at(e, before_cursor))?)
        } else if let Token::Symbol(Symbol::Minus) = first {
            let next = buffer.parse::<Literal>()?;

            match next {
                Literal::Int(value) => Ok(Literal::Int(-value)),
                Literal::Float(value) => Ok(Literal::Float(-value)),
                _ => Err(buffer.error_at(ExprParseError::CannotParseIntoIdent, before_cursor)),
            }
        } else {
            Err(buffer.error_at(ExprParseError::CannotParseIntoIdent, before_cursor))
        }
    }
}

impl Peek for Literal {
    fn peek(buffer: &ParseBuffer) -> bool {
        let mut iter = buffer.get_tokens().iter();
        let mut current = iter.next();
        let mut minus = false;

        while let Some(Token::Symbol(Symbol::Minus)) = current {
            minus = true;
            current = iter.next();
        }

        if let Some(Token::Lit(lit)) = current {
            if minus {
                matches!(lit, Lit::Int(_) | Lit::Float(_))
            } else {
                true
            }
        } else {
            false
        }
    }
}

#[test]
fn test_lit() {
    use crate::query::parse::TokenStream;
    use std::str::FromStr;
    let token_stream = TokenStream::from_str("---114514").unwrap();

    let lit: Literal = token_stream.parse().unwrap();

    assert_eq!(lit, Literal::Int(-114514))
}
