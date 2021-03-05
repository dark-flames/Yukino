use crate::query::expr::expression::Expression;
use crate::query::parse::{Parse, ParseBuffer, Error, Token, Symbol, Keyword};
use crate::query::expr::error::ExprParseError;
use crate::query::expr::precedence::Precedence;

#[derive(Debug, Eq, PartialEq)]
pub enum ArithmeticOrLogicalExpression {
    Binary(BinaryExpression),
    Unary(UnaryExpression)
}

pub enum UnaryOperator {
    BitInverse,
    Not
}

impl Parse for UnaryOperator {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let cursor = buffer.cursor();
        match buffer.pop_token()? {
            Token::Symbol(Symbol::Tilde) => Ok(UnaryOperator::BitInverse),
            Token::Keyword(Keyword::Not) => Ok(UnaryOperator::Not),
            _ => Err(buffer.error_at(ExprParseError::CannotParseIntoUnaryOperator, cursor))
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
            UnaryOperator::Not => Precedence::Not
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
            UnaryOperator::Not => UnaryExpression::Not(boxed)
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
        let operator: UnaryOperator = buffer.parse()?;

        let right = Expression::parse_right_with_precedence(
            buffer,
            operator.precedence()
        )?;

        Ok(operator.with_expr(right))
    }

    fn peek(buffer: &ParseBuffer) -> bool {
        buffer.peek::<UnaryOperator>()
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Eq, PartialEq)]
pub enum BinaryExpression {
    BitXor(Box<Expression>, Box<Expression>),
    Multi(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    BitLeftShift(Box<Expression>, Box<Expression>),
    BitRightShift(Box<Expression>, Box<Expression>),
    BitAnd(Box<Expression>, Box<Expression>),
    BitOr(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>),
    EQ(Box<Expression>, Box<Expression>),
    NEQ(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
}
