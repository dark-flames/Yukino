use crate::query::expr::error::ExprParseError;
use crate::query::expr::expression::Expression;
use crate::query::parse::{Error, Parse, ParseBuffer, Symbol, Token};

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionCall {
    ident: String,
    parameters: Vec<Expression>,
}

impl Parse for FunctionCall {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let cursor = buffer.cursor();

        if let Token::Ident(ident) = buffer.pop_token()? {
            if let Token::Symbol(Symbol::ParenLeft) = buffer.pop_token()? {
                let mut parameters = vec![];
                while buffer.peek::<Expression>() {
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

    fn peek(buffer: &ParseBuffer) -> bool {
        let mut buffer_cloned = buffer.clone();

        matches!(buffer_cloned.parse::<FunctionCall>(), Ok(_))
    }
}
