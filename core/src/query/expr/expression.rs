use crate::query::expr::function::FunctionCall;
use crate::query::expr::ident::DatabaseIdent;
use crate::query::expr::literal::Literal;
use crate::query::expr::binary::{BinaryExpression, BinaryOperator};
use crate::query::expr::precedence::Precedence;
use crate::query::parse::{Error, Parse, ParseBuffer, Token, Symbol};
use crate::query::expr::error::ExprParseError;
use crate::query::expr::unary::{UnaryExpression, UnaryOperator};

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Ident(DatabaseIdent),
    Function(FunctionCall),
    Literal(Literal)
}

impl Parse for Expression {
    fn parse(_buffer: &mut ParseBuffer) -> Result<Self, Error> {
        unimplemented!()
    }

    fn peek(_buffer: &ParseBuffer) -> bool {
        unimplemented!()
    }
}

impl Expression {
    pub fn parse_item_with_precedence(
        buffer: &mut ParseBuffer,
        precedence: Precedence,
    ) -> Result<Option<Self>, Error> {
        let head = buffer.cursor();
        let mut result = if buffer.peek_token(Token::Symbol(Symbol::ParenLeft)) {
            let offset = buffer.get_tokens().iter().position(
                |token| matches!(token, Token::Symbol(Symbol::ParenRight))
            ).ok_or_else(|| buffer.error_at(
                ExprParseError::CannotFindRightParen,
                head
            ))?;

            let mut new_buffer = buffer.split_at(offset);

            let expr = new_buffer.parse::<Expression>()?;

            buffer.pop_token()?;

            expr
        } else if precedence <= Precedence::Lit && buffer.peek::<Literal>() {
            Expression::Literal(buffer.parse()?)
        } else if precedence <= Precedence::FunctionCall && buffer.peek::<FunctionCall>() {
            Expression::Function(buffer.parse()?)
        } else if precedence <= Precedence::Ident && buffer.peek::<DatabaseIdent>() {
            Expression::Ident(buffer.parse()?)
        } else if buffer.peek::<UnaryOperator>() {
            let operator = UnaryOperator::peek_operator(buffer)?;

            if precedence <= operator.precedence() {
                Expression::Unary(buffer.parse()?)
            } else {
                return Ok(None)
            }
        } else {
            return Err(buffer.error_at(
                ExprParseError::CannotParseIntoExpression,
                head
            ))
        };

        while !buffer.is_empty() {
            if let Ok(operator) = BinaryOperator::peek_operator(buffer) {
                if precedence <= operator.precedence() {
                    result = Expression::Binary(
                        BinaryExpression::parse_right_side(buffer, result)?
                    );
                } else {
                    break;
                }
            } else {
                break;
            }
        };

        Ok(Some(result))
    }
}
