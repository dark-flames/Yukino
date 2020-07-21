use crate::query::Expression;
use syn::parse::{Parse, ParseBuffer};
use syn::Error;

pub enum MathematicalExpression {
    Parentheses(Box<Expression>),
    BitInverse(Box<Expression>),
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
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum MathematicalPrecedence {
    Parentheses,
    BitInverse,
    BitXor,
    Term,
    Add,
    BitShift,
    BitAnd,
    BitOr,
    Comparison,
}

impl MathematicalPrecedence {
    pub fn of(expr: &MathematicalExpression) -> Self {
        match expr {
            MathematicalExpression::Parentheses(_) => MathematicalPrecedence::Parentheses,
            MathematicalExpression::BitInverse(_) => MathematicalPrecedence::BitInverse,
            MathematicalExpression::BitXor(_, _) => MathematicalPrecedence::BitXor,
            MathematicalExpression::Multi(_, _)
            | MathematicalExpression::Div(_, _)
            | MathematicalExpression::Mod(_, _) => MathematicalPrecedence::Term,
            MathematicalExpression::Add(_, _) | MathematicalExpression::Sub(_, _) => {
                MathematicalPrecedence::Add
            }
            MathematicalExpression::BitLeftShift(_, _)
            | MathematicalExpression::BitRightShift(_, _) => MathematicalPrecedence::BitShift,
            MathematicalExpression::BitAnd(_, _) => MathematicalPrecedence::BitAnd,
            MathematicalExpression::BitOr(_, _) => MathematicalPrecedence::BitOr,
            MathematicalExpression::GT(_, _)
            | MathematicalExpression::LT(_, _)
            | MathematicalExpression::GTE(_, _)
            | MathematicalExpression::LTE(_, _)
            | MathematicalExpression::EQ(_, _)
            | MathematicalExpression::NEQ(_, _) => MathematicalPrecedence::Comparison,
        }
    }
}

impl Parse for MathematicalExpression {
    fn parse<'a>(_input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        unimplemented!()
    }
}
