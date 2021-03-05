use crate::query::expr::error::ExprParseError;
use crate::query::expr::expression::Expression;
use crate::query::expr::precedence::Precedence;
use crate::query::parse::{Error, Keyword, Parse, ParseBuffer, Symbol, Token};

#[derive(Debug, Eq, PartialEq)]
pub enum ArithmeticOrLogicalExpression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
}

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
        let operator: UnaryOperator = buffer.parse()?;

        let right = Expression::parse_right_with_precedence(buffer, operator.precedence())?;

        Ok(operator.with_expr(right))
    }

    fn peek(buffer: &ParseBuffer) -> bool {
        buffer.peek::<UnaryOperator>()
    }
}

#[allow(clippy::upper_case_acronyms)]
pub enum BinaryOperator {
    BitXor,
    Multi,
    Mod,
    Div,
    Add,
    Sub,
    BitLeftShift,
    BitRightShift,
    BitAnd,
    BitOr,
    GT,
    LT,
    GTE,
    LTE,
    EQ,
    NEQ,
    And,
    Or,
    Xor,
}

impl Parse for BinaryOperator {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let cursor = buffer.cursor();
        match buffer.pop_token()? {
            Token::Symbol(Symbol::Caret) => Ok(BinaryOperator::BitXor),
            Token::Symbol(Symbol::Mul) => Ok(BinaryOperator::Multi),
            Token::Symbol(Symbol::Mod) => Ok(BinaryOperator::Mod),
            Token::Symbol(Symbol::Div) => Ok(BinaryOperator::Div),
            Token::Symbol(Symbol::Add) => Ok(BinaryOperator::Add),
            Token::Symbol(Symbol::Minus) => Ok(BinaryOperator::Sub),
            Token::Symbol(Symbol::LeftShift) => Ok(BinaryOperator::BitLeftShift),
            Token::Symbol(Symbol::RightShift) => Ok(BinaryOperator::BitRightShift),
            Token::Symbol(Symbol::And) => Ok(BinaryOperator::BitAnd),
            Token::Symbol(Symbol::Or) => Ok(BinaryOperator::BitOr),
            Token::Symbol(Symbol::Greater) => Ok(BinaryOperator::GT),
            Token::Symbol(Symbol::Less) => Ok(BinaryOperator::LT),
            Token::Symbol(Symbol::GreaterEqual) => Ok(BinaryOperator::GTE),
            Token::Symbol(Symbol::LessEqual) => Ok(BinaryOperator::LTE),
            Token::Symbol(Symbol::Equal) => Ok(BinaryOperator::EQ),
            Token::Symbol(Symbol::NotEqual) => Ok(BinaryOperator::NEQ),
            Token::Keyword(Keyword::And) => Ok(BinaryOperator::And),
            Token::Keyword(Keyword::Or) => Ok(BinaryOperator::Or),
            Token::Keyword(Keyword::Xor) => Ok(BinaryOperator::Xor),
            _ => Err(buffer.error_at(ExprParseError::CannotParseIntoBinaryOperator, cursor)),
        }
    }

    fn peek(buffer: &ParseBuffer) -> bool {
        let mut buffer_cloned = buffer.clone();

        matches!(buffer_cloned.parse::<BinaryOperator>(), Ok(_))
    }
}

#[allow(dead_code)]
impl BinaryOperator {
    pub fn precedence(&self) -> Precedence {
        match self {
            BinaryOperator::BitXor => Precedence::BitXor,
            BinaryOperator::Multi | BinaryOperator::Mod | BinaryOperator::Div => Precedence::Term,
            BinaryOperator::Add | BinaryOperator::Sub => Precedence::Add,
            BinaryOperator::BitLeftShift | BinaryOperator::BitRightShift => Precedence::BitShift,
            BinaryOperator::BitAnd => Precedence::BitAnd,
            BinaryOperator::BitOr => Precedence::BitOr,
            BinaryOperator::GT | BinaryOperator::LT
                | BinaryOperator::GTE
                | BinaryOperator::LTE
                | BinaryOperator::EQ
                | BinaryOperator::NEQ => Precedence::Comparison,
            BinaryOperator::And => Precedence::And,
            BinaryOperator::Or => Precedence::Or,
            BinaryOperator::Xor => Precedence::Xor
        }
    }

    pub fn peek_operator(buffer: &ParseBuffer) -> Result<Self, Error> {
        let mut buffer_cloned = buffer.clone();

        buffer_cloned.parse::<BinaryOperator>()
    }

    pub fn with_expr(&self, left: Expression, right: Expression) -> BinaryExpression {
        let boxed_left = Box::new(left);
        let boxed_right = Box::new(right);

        match self {
            BinaryOperator::BitXor => BinaryExpression::BitXor(boxed_left, boxed_right),
            BinaryOperator::Multi => BinaryExpression::Multi(boxed_left, boxed_right),
            BinaryOperator::Mod => BinaryExpression::Mod(boxed_left, boxed_right),
            BinaryOperator::Div => BinaryExpression::Div(boxed_left, boxed_right),
            BinaryOperator::Add => BinaryExpression::Add(boxed_left, boxed_right),
            BinaryOperator::Sub => BinaryExpression::Sub(boxed_left, boxed_right),
            BinaryOperator::BitLeftShift => BinaryExpression::BitLeftShift(boxed_left, boxed_right),
            BinaryOperator::BitRightShift => BinaryExpression::BitRightShift(boxed_left, boxed_right),
            BinaryOperator::BitAnd => BinaryExpression::BitAnd(boxed_left, boxed_right),
            BinaryOperator::BitOr => BinaryExpression::BitOr(boxed_left, boxed_right),
            BinaryOperator::GT => BinaryExpression::GT(boxed_left, boxed_right),
            BinaryOperator::LT => BinaryExpression::LT(boxed_left, boxed_right),
            BinaryOperator::GTE => BinaryExpression::GTE(boxed_left, boxed_right),
            BinaryOperator::LTE => BinaryExpression::LTE(boxed_left, boxed_right),
            BinaryOperator::EQ => BinaryExpression::EQ(boxed_left, boxed_right),
            BinaryOperator::NEQ => BinaryExpression::NEQ(boxed_left, boxed_right),
            BinaryOperator::And => BinaryExpression::And(boxed_left, boxed_right),
            BinaryOperator::Or => BinaryExpression::Or(boxed_left, boxed_right),
            BinaryOperator::Xor => BinaryExpression::Xor(boxed_left, boxed_right),
        }
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
