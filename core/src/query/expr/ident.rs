use crate::query::expr::error::ExprParseError;
use crate::query::parse::{Error, Parse, ParseBuffer, Symbol, Token};

#[derive(Eq, PartialEq, Debug)]
pub struct DatabaseIdent {
    segments: Vec<String>,
}

impl Parse for DatabaseIdent {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let mut segments = vec![];

        if !buffer.is_empty() {
            while let Token::Ident(ident) = buffer.pop_token()? {
                segments.push(ident.to_string());

                if buffer.is_empty() {
                    return Ok(DatabaseIdent { segments });
                } else if let Token::Symbol(Symbol::Dot) = buffer.pop_token()? {
                    continue;
                } else {
                    return Ok(DatabaseIdent { segments });
                }
            }
        }

        Err(buffer.error(ExprParseError::CannotParseIntoIdent))
    }

    fn peek(buffer: &ParseBuffer) -> bool {
        let mut iter = buffer.get_token().iter();
        let mut matched = false;

        if !buffer.is_empty() {
            while let Some(Token::Ident(_)) = iter.next() {
                matched = true;
                if buffer.is_empty() {
                    break;
                } else if let Some(Token::Symbol(Symbol::Dot)) = iter.next() {
                    matched = false
                } else {
                    break;
                }
            }
        }

        matched
    }
}

#[test]
fn test_ident() {
    use crate::query::parse::TokenStream;
    use std::str::FromStr;
    let token_stream = TokenStream::from_str("a.b.c").unwrap();

    let ident: DatabaseIdent = token_stream.parse().unwrap();

    assert_eq!(
        ident,
        DatabaseIdent {
            segments: vec!["a".to_string(), "b".to_string(), "c".to_string()]
        }
    )
}
