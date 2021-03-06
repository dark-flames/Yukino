use crate::query::expr::error::ExprParseError;
use crate::query::expr::precedence::Precedence;
use crate::query::expr::Expression;
use crate::query::parse::{Error, Keyword, Parse, ParseBuffer, Symbol, Token};

pub enum UnaryOperator {
    BitInverse,
    Not,
}

impl Parse for UnaryOperator {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let cursor = buffer.cursor();
        match buffer.pop_token()? {
            Token::Symbol(Symbol::Tilde) => Ok(UnaryOperator::BitInverse),
            Token::Keyword(Keyword::Not) => Ok(UnaryOperator::Not),
            _ => Err(buffer.error_at(ExprParseError::CannotParseIntoUnaryOperator, cursor)),
        }
    }

    fn peek(buffer: &ParseBuffer) -> bool {
        buffer.peek_token(Token::Symbol(Symbol::Tilde))
            || buffer.peek_token(Token::Keyword(Keyword::Not))
    }
}

impl UnaryOperator {
    pub fn precedence(&self) -> Precedence {
        match self {
            UnaryOperator::BitInverse => Precedence::BitInverse,
            UnaryOperator::Not => Precedence::Not,
        }
    }

    pub fn peek_operator(buffer: &ParseBuffer) -> Result<Self, Error> {
        let mut buffer_cloned = buffer.clone();

        buffer_cloned.parse::<UnaryOperator>()
    }

    pub fn with_expr(&self, expr: Expression) -> UnaryExpression {
        let boxed = Box::new(expr);

        match self {
            UnaryOperator::BitInverse => UnaryExpression::BitInverse(boxed),
            UnaryOperator::Not => UnaryExpression::Not(boxed),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnaryExpression {
    BitInverse(Box<Expression>),
    Not(Box<Expression>),
}

impl Parse for UnaryExpression {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let head = buffer.cursor();
        let operator: UnaryOperator = buffer.parse()?;

        let right = Expression::parse_item_with_precedence(buffer, operator.precedence())?
            .ok_or_else(|| buffer.error_at(ExprParseError::CannotParseIntoExpression, head))?;

        Ok(operator.with_expr(right))
    }

    fn peek(buffer: &ParseBuffer) -> bool {
        buffer.peek::<UnaryOperator>()
    }
}
