use crate::query::expr::error::ExprParseError;
use crate::query::expr::expression::Expression;
use crate::query::expr::precedence::Precedence;
use crate::query::parse::{Error, Keyword, Parse, ParseBuffer, Peek, Symbol, Token};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
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
}

impl Peek for BinaryOperator {
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
            BinaryOperator::GT
            | BinaryOperator::LT
            | BinaryOperator::GTE
            | BinaryOperator::LTE
            | BinaryOperator::EQ
            | BinaryOperator::NEQ => Precedence::Comparison,
            BinaryOperator::And => Precedence::And,
            BinaryOperator::Or => Precedence::Or,
            BinaryOperator::Xor => Precedence::Xor,
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
            BinaryOperator::BitRightShift => {
                BinaryExpression::BitRightShift(boxed_left, boxed_right)
            }
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

impl BinaryExpression {
    pub fn operator(&self) -> BinaryOperator {
        match self {
            BinaryExpression::BitXor(_, _) => BinaryOperator::BitXor,
            BinaryExpression::Multi(_, _) => BinaryOperator::Multi,
            BinaryExpression::Mod(_, _) => BinaryOperator::Mod,
            BinaryExpression::Div(_, _) => BinaryOperator::Div,
            BinaryExpression::Add(_, _) => BinaryOperator::Add,
            BinaryExpression::Sub(_, _) => BinaryOperator::Sub,
            BinaryExpression::BitLeftShift(_, _) => BinaryOperator::BitLeftShift,
            BinaryExpression::BitRightShift(_, _) => BinaryOperator::BitRightShift,
            BinaryExpression::BitAnd(_, _) => BinaryOperator::BitAnd,
            BinaryExpression::BitOr(_, _) => BinaryOperator::BitOr,
            BinaryExpression::GT(_, _) => BinaryOperator::GT,
            BinaryExpression::LT(_, _) => BinaryOperator::LT,
            BinaryExpression::GTE(_, _) => BinaryOperator::GTE,
            BinaryExpression::LTE(_, _) => BinaryOperator::LTE,
            BinaryExpression::EQ(_, _) => BinaryOperator::EQ,
            BinaryExpression::NEQ(_, _) => BinaryOperator::NEQ,
            BinaryExpression::And(_, _) => BinaryOperator::And,
            BinaryExpression::Or(_, _) => BinaryOperator::Or,
            BinaryExpression::Xor(_, _) => BinaryOperator::Xor,
        }
    }

    pub fn parse_right_side(
        buffer: &mut ParseBuffer,
        left: Expression,
    ) -> Result<BinaryExpression, Error> {
        let head = buffer.cursor();
        let operator: BinaryOperator = buffer.parse()?;

        let right = Expression::parse_item_with_precedence(buffer, operator.precedence())?
            .ok_or_else(|| buffer.error_at(ExprParseError::CannotParseIntoExpression, head))?;

        Ok(operator.with_expr(left, right))
    }
}
