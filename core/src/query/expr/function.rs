use crate::query::expr::error::ExprParseError;
use crate::query::expr::expression::Expression;
use crate::query::parse::{Error, Parse, ParseBuffer, Peek, Symbol, Token};

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionCall {
    pub ident: String,
    pub parameters: Vec<Expression>,
}

impl Parse for FunctionCall {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let cursor = buffer.cursor();

        if let Token::Ident(ident) = buffer.pop_token()? {
            if let Token::Symbol(Symbol::ParenLeft) = buffer.pop_token()? {
                let mut parameters = vec![];
                loop {
                    let expr = buffer.parse()?;
                    parameters.push(expr);

                    match buffer.pop_token()? {
                        Token::Symbol(Symbol::Comma) => continue,
                        Token::Symbol(Symbol::ParenRight) => break,
                        _ => {
                            return Err(
                                buffer.error_at(ExprParseError::CannotParseIntoFunction, cursor)
                            )
                        }
                    }
                }

                Ok(FunctionCall {
                    ident: ident.to_string(),
                    parameters,
                })
            } else {
                Err(buffer.error_at(ExprParseError::CannotParseIntoFunction, cursor))
            }
        } else {
            Err(buffer.error_at(ExprParseError::CannotParseIntoFunction, cursor))
        }
    }
}

impl Peek for FunctionCall {
    fn peek(buffer: &ParseBuffer) -> bool {
        let mut tokens = buffer.get_tokens().iter();

        matches!(tokens.next(), Some(Token::Ident(_)))
            && matches!(tokens.next(), Some(Token::Symbol(Symbol::ParenLeft)))
            && tokens.any(|token| matches!(token, Token::Symbol(Symbol::ParenRight)))
    }
}

#[test]
fn test_function_peek() {
    use crate::query::parse::TokenStream;
    use std::str::FromStr;

    let token_stream =
        TokenStream::from_str("test(table.column.a, \"やりますねぇ\", false)").unwrap();

    assert!(token_stream.peek::<FunctionCall>())
}
