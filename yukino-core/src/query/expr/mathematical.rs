use crate::query::{Expression, IdentExpression, Peekable, Value};
use proc_macro2::Span;
use syn::parse::{Parse, ParseBuffer};
use syn::{token, Error, Token};

pub enum MathematicalExpression {
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

#[derive(Debug)]
pub enum BinaryOperator {
    BitXor(Token![^]),
    Multi(Token![*]),
    Mod(Token![%]),
    Div(Token![/]),
    Add(Token![+]),
    Sub(Token![-]),
    BitLeftShift(Token![<<]),
    BitRightShift(Token![>>]),
    BitAnd(Token![&]),
    BitOr(Token![|]),
    GT(Token![>]),
    LT(Token![<]),
    GTE(Token![>=]),
    LTE(Token![<=]),
    EQ(Token![==]),
    NEQ(Token![!=]),
}

pub enum UnaryOperator {
    BitInverse(Token![~]),
}

impl Parse for BinaryOperator {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![^]) {
            input.parse().map(BinaryOperator::BitXor)
        } else if input.peek(Token![*]) {
            input.parse().map(BinaryOperator::Multi)
        } else if input.peek(Token![/]) {
            input.parse().map(BinaryOperator::Div)
        } else if input.peek(Token![%]) {
            input.parse().map(BinaryOperator::Mod)
        } else if input.peek(Token![+]) {
            input.parse().map(BinaryOperator::Add)
        } else if input.peek(Token![-]) {
            input.parse().map(BinaryOperator::Sub)
        } else if input.peek(Token![>>]) {
            input.parse().map(BinaryOperator::BitRightShift)
        } else if input.peek(Token![<<]) {
            input.parse().map(BinaryOperator::BitLeftShift)
        } else if input.peek(Token![&]) {
            input.parse().map(BinaryOperator::BitAnd)
        } else if input.peek(Token![|]) {
            input.parse().map(BinaryOperator::BitOr)
        } else if input.peek(Token![<]) {
            input.parse().map(BinaryOperator::LT)
        } else if input.peek(Token![>]) {
            input.parse().map(BinaryOperator::GT)
        } else if input.peek(Token![<=]) {
            input.parse().map(BinaryOperator::LTE)
        } else if input.peek(Token![>=]) {
            input.parse().map(BinaryOperator::GTE)
        } else if input.peek(Token![==]) {
            input.parse().map(BinaryOperator::EQ)
        } else if input.peek(Token![!=]) {
            input.parse().map(BinaryOperator::NEQ)
        } else {
            Err(Error::new(
                Span::call_site(),
                "Cannot parse into an binary operator",
            ))
        }
    }
}

impl Peekable for BinaryOperator {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Token![^])
        | input.peek(Token![*])
        | input.peek(Token![/])
        | input.peek(Token![%])
        | input.peek(Token![+])
        | input.peek(Token![-])
        | input.peek(Token![>>])
        | input.peek(Token![<<])
        | input.peek(Token![&])
        | input.peek(Token![|])
        | input.peek(Token![<])
        | input.peek(Token![>])
        | input.peek(Token![<=])
        | input.peek(Token![>=])
        | input.peek(Token![==])
        | input.peek(Token![!=])
    }
}

impl Parse for UnaryOperator {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![~]) {
            input.parse().map(UnaryOperator::BitInverse)
        } else {
            Err(Error::new(
                Span::call_site(),
                "Cannot parse into an Unary operator",
            ))
        }
    }
}

impl Peekable for UnaryOperator {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Token![~])
    }
}

impl BinaryOperator {
    pub fn construct_expr(&self, left: Expression, right: Expression) -> MathematicalExpression {
        let left_box = Box::new(left);
        let right_box = Box::new(right);
        match self {
            BinaryOperator::BitXor(_) => MathematicalExpression::BitXor(left_box, right_box),
            BinaryOperator::Multi(_) => MathematicalExpression::Multi(left_box, right_box),
            BinaryOperator::Div(_) => MathematicalExpression::Div(left_box, right_box),
            BinaryOperator::Mod(_) => MathematicalExpression::Mod(left_box, right_box),
            BinaryOperator::Add(_) => MathematicalExpression::Add(left_box, right_box),
            BinaryOperator::Sub(_) => MathematicalExpression::Sub(left_box, right_box),
            BinaryOperator::BitLeftShift(_) => {
                MathematicalExpression::BitLeftShift(left_box, right_box)
            }
            BinaryOperator::BitRightShift(_) => {
                MathematicalExpression::BitRightShift(left_box, right_box)
            }
            BinaryOperator::BitAnd(_) => MathematicalExpression::BitAnd(left_box, right_box),
            BinaryOperator::BitOr(_) => MathematicalExpression::BitOr(left_box, right_box),
            BinaryOperator::GT(_) => MathematicalExpression::GT(left_box, right_box),
            BinaryOperator::LT(_) => MathematicalExpression::LT(left_box, right_box),
            BinaryOperator::GTE(_) => MathematicalExpression::GTE(left_box, right_box),
            BinaryOperator::LTE(_) => MathematicalExpression::LTE(left_box, right_box),
            BinaryOperator::EQ(_) => MathematicalExpression::EQ(left_box, right_box),
            BinaryOperator::NEQ(_) => MathematicalExpression::NEQ(left_box, right_box),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            BinaryOperator::BitXor(_) => Precedence::BitXor,
            BinaryOperator::Multi(_) | BinaryOperator::Div(_) | BinaryOperator::Mod(_) => {
                Precedence::Term
            }
            BinaryOperator::Add(_) | BinaryOperator::Sub(_) => Precedence::Add,
            BinaryOperator::BitLeftShift(_) | BinaryOperator::BitRightShift(_) => {
                Precedence::BitShift
            }
            BinaryOperator::BitAnd(_) => Precedence::BitAnd,
            BinaryOperator::BitOr(_) => Precedence::BitOr,
            BinaryOperator::GT(_)
            | BinaryOperator::LT(_)
            | BinaryOperator::GTE(_)
            | BinaryOperator::LTE(_)
            | BinaryOperator::EQ(_)
            | BinaryOperator::NEQ(_) => Precedence::Comparison,
        }
    }
}

impl UnaryOperator {
    pub fn construct_expr(&self, expr: Expression) -> MathematicalExpression {
        let boxed_expr = Box::new(expr);

        match self {
            UnaryOperator::BitInverse(_) => MathematicalExpression::BitInverse(boxed_expr),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            UnaryOperator::BitInverse(_) => Precedence::BitInverse,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Precedence {
    None,
    Comparison,
    BitOr,
    BitAnd,
    BitShift,
    Add,
    Term,
    BitXor,
    BitInverse,
    Parentheses,
}

impl Precedence {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> Option<Self> {
        if input.peek(Token![^]) {
            Some(Precedence::BitXor)
        } else if input.peek(Token![*]) | input.peek(Token![/]) | input.peek(Token![%]) {
            Some(Precedence::Term)
        } else if input.peek(Token![+]) | input.peek(Token![-]) {
            Some(Precedence::Add)
        } else if input.peek(Token![>>]) | input.peek(Token![<<]) {
            Some(Precedence::BitShift)
        } else if input.peek(Token![&]) {
            Some(Precedence::BitAnd)
        } else if input.peek(Token![|]) {
            Some(Precedence::BitOr)
        } else if input.peek(Token![<])
            | input.peek(Token![>])
            | input.peek(Token![<=])
            | input.peek(Token![>=])
            | input.peek(Token![==])
            | input.peek(Token![!=])
        {
            Some(Precedence::Comparison)
        } else {
            None
        }
    }
}

impl MathematicalExpression {
    pub fn parse_right_expression<'a>(
        input: &'a ParseBuffer<'a>,
        operator_precedence: Precedence,
    ) -> Result<Expression, Error> {
        let result = if input.peek(token::Paren) {
            Expression::parse_item(input)
        } else if let Ok(unary_operator) = input.parse::<UnaryOperator>() {
            let right =
                MathematicalExpression::parse_right_expression(input, unary_operator.precedence())?;

            Ok(Expression::MathematicalExpr(
                unary_operator.construct_expr(right),
            ))
        } else if let Ok(value) = input.parse::<Value>() {
            Ok(Expression::Value(value))
        } else if let Ok(ident) = input.parse::<IdentExpression>() {
            Ok(Expression::IdentExpr(ident))
        } else {
            Err(Error::new(
                Span::call_site(),
                "Cannot parse into expression or unary operator",
            ))
        }?;

        let next_binary_operator_precedence = Precedence::peek(input);

        match next_binary_operator_precedence {
            Some(next_precedence) if next_precedence > operator_precedence => {
                let operator = input.parse::<BinaryOperator>()?;
                let next_expr = Self::parse_right_expression(input, operator.precedence())?;

                Ok(Expression::MathematicalExpr(
                    operator.construct_expr(result, next_expr),
                ))
            }
            _ => Ok(result),
        }
    }

    pub fn parse_operator_and_right_expression<'a>(
        input: &'a ParseBuffer<'a>,
        left: Expression,
    ) -> Result<Expression, Error> {
        let mut result = left;

        while !input.is_empty() {
            let operator = match input.parse::<BinaryOperator>() {
                Ok(o) => o,
                _ => break,
            };

            result = Expression::MathematicalExpr(operator.construct_expr(
                result,
                Self::parse_right_expression(input, operator.precedence())?,
            ))
        }

        Ok(result)
    }

    pub fn parse_into_expression<'a>(input: &'a ParseBuffer<'a>) -> Result<Expression, Error> {
        Self::parse_operator_and_right_expression(
            input,
            Self::parse_right_expression(input, Precedence::None)?,
        )
    }
}

impl Parse for MathematicalExpression {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        match Self::parse_operator_and_right_expression(
            input,
            Self::parse_right_expression(input, Precedence::None)?,
        )? {
            Expression::MathematicalExpr(mathematical_expr) => Ok(mathematical_expr),
            _ => Err(Error::new(Span::call_site(),"Not a mathematical expression"))
        }
    }
}

impl Peekable for MathematicalExpression {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        Precedence::peek(input).is_some()
    }
}
