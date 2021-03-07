use crate::query::expr::error::ExprParseError;
use crate::query::parse::{Error, Parse, ParseBuffer, Symbol, Token};

#[derive(Eq, PartialEq, Debug)]
pub struct DatabaseIdent {
    pub segments: Vec<String>,
}

impl Parse for DatabaseIdent {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let cursor = buffer.cursor();
        let mut segments = vec![];

        if !buffer.is_empty() {
            while let Some(token) = buffer.get_token() {
                let any = match token {
                    Token::Ident(ident) => {
                        segments.push(ident.to_string());
                        false
                    },
                    Token::Symbol(Symbol::Mul) => {
                        segments.push("*".to_string());
                        true
                    },
                    _ => break
                };

                buffer.pop_token()?;

                if buffer.is_empty() {
                    return Ok(DatabaseIdent { segments });
                } else if let Some(Token::Symbol(Symbol::Dot)) = buffer.get_token() {
                    buffer.pop_token()?;
                    if any {
                        return Err(buffer.error_at(ExprParseError::UnexpectedAny, cursor))
                    }
                    continue;
                } else {
                    return Ok(DatabaseIdent { segments });
                }
            }
        }

        Err(buffer.error_at(ExprParseError::CannotParseIntoIdent, cursor))
    }

    fn peek(buffer: &ParseBuffer) -> bool {
        let mut iter = buffer.get_tokens().iter();
        let mut matched = false;

        if !buffer.is_empty() {
            while let Some(token) = iter.next() {
                let any = match token {
                    Token::Ident(_) => false,
                    Token::Symbol(Symbol::Mul) => true,
                    _ => break
                };

                matched = true;
                if buffer.is_empty() {
                    break;
                } else if let Some(Token::Symbol(Symbol::Dot)) = iter.next() {
                    if any {
                        return false;
                    };

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
