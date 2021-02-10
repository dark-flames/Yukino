use crate::query::expr::expression::Expression;
use syn::parse::{Parse, ParseBuffer};
use syn::{Token, Error, Ident as IdentMark, token::Paren};
use proc_macro2::Ident;
use crate::query::expr::helper::Peekable;
use crate::query::expr::precedence::Precedence;
use crate::query::expr::literal::Literal;
use crate::query::expr::ident::DatabaseIdent;

pub enum ArithmeticOrLogicalExpression {
    BitInverse(Box<Expression>),
    Not(Box<Expression>),
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
    Xor(Box<Expression>, Box<Expression>)
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
    And,
    Or,
    Xor,
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
        } else if input.peek(Token![<=]) {
            input.parse().map(BinaryOperator::LTE)
        } else if input.peek(Token![>=]) {
            input.parse().map(BinaryOperator::GTE)
        } else if input.peek(Token![<]) {
            input.parse().map(BinaryOperator::LT)
        } else if input.peek(Token![>]) {
            input.parse().map(BinaryOperator::GT)
        } else if input.peek(Token![==]) {
            input.parse().map(BinaryOperator::EQ)
        } else if input.peek(Token![!=]) {
            input.parse().map(BinaryOperator::NEQ)
        } else if input.peek(IdentMark) {
            let ident: Ident = input.parse()?;
            let ident_str = ident.to_string().to_lowercase();
            match ident_str.as_str() {
                "and" => Ok(BinaryOperator::And),
                "or" => Ok(BinaryOperator::Or),
                "xor" => Ok(BinaryOperator::Xor),
                _ => Err(input.error("Unexpected binary logical operator")),
            }
        } else if input.peek(Token![||]) {
            Ok(BinaryOperator::Or)
        } else if input.peek(Token![&&]) {
            Ok(BinaryOperator::And)
        } else {
            Err(input.error("Cannot parse into an binary operator"))
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
            | match input.fork().parse::<Ident>() {
                Ok(ident) => matches!(ident.to_string().to_lowercase().as_str(), "and" | "or" | "xor"),
                _ => false,
            }
    }
}

pub enum UnaryOperator {
    BitInverse(Token![~]),
    Not,
}

impl Parse for UnaryOperator {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![~]) {
            input.parse().map(UnaryOperator::BitInverse)
        } else if input.peek(IdentMark) {
            let ident: Ident = input.parse()?;
            let ident_str = ident.to_string().to_lowercase();
            match ident_str.as_str() {
                "not" => Ok(UnaryOperator::Not),
                _ => Err(input.error("Cannot parse into an Unary operator")),
            }
        } else {
            Err(input.error("Cannot parse into an Unary operator"))
        }
    }
}

impl Peekable for UnaryOperator {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Token![~]) || {
            let fork = input.fork();
            if fork.peek(IdentMark) {}
            match fork.parse::<IdentMark>() {
                Ok(ident) => matches!(ident.to_string().to_lowercase().as_str(), "not"),
                _ => false,
            }
        }
    }
}

impl UnaryOperator {
    pub fn construct_expr(&self, expr: Expression) -> ArithmeticOrLogicalExpression {
        let boxed_expr = Box::new(expr);

        match self {
            UnaryOperator::BitInverse(_) => ArithmeticOrLogicalExpression::BitInverse(boxed_expr),
            UnaryOperator::Not => ArithmeticOrLogicalExpression::Not(boxed_expr),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            UnaryOperator::BitInverse(_) => Precedence::BitInverse,
            UnaryOperator::Not => Precedence::Not,
        }
    }
}

impl BinaryOperator {
    pub fn construct_expr(
        &self,
        left: Expression,
        right: Expression,
    ) -> ArithmeticOrLogicalExpression {
        let left_box = Box::new(left);
        let right_box = Box::new(right);
        match self {
            BinaryOperator::BitXor(_) => ArithmeticOrLogicalExpression::BitXor(left_box, right_box),
            BinaryOperator::Multi(_) => ArithmeticOrLogicalExpression::Multi(left_box, right_box),
            BinaryOperator::Div(_) => ArithmeticOrLogicalExpression::Div(left_box, right_box),
            BinaryOperator::Mod(_) => ArithmeticOrLogicalExpression::Mod(left_box, right_box),
            BinaryOperator::Add(_) => ArithmeticOrLogicalExpression::Add(left_box, right_box),
            BinaryOperator::Sub(_) => ArithmeticOrLogicalExpression::Sub(left_box, right_box),
            BinaryOperator::BitLeftShift(_) => {
                ArithmeticOrLogicalExpression::BitLeftShift(left_box, right_box)
            }
            BinaryOperator::BitRightShift(_) => {
                ArithmeticOrLogicalExpression::BitRightShift(left_box, right_box)
            }
            BinaryOperator::BitAnd(_) => ArithmeticOrLogicalExpression::BitAnd(left_box, right_box),
            BinaryOperator::BitOr(_) => ArithmeticOrLogicalExpression::BitOr(left_box, right_box),
            BinaryOperator::GT(_) => ArithmeticOrLogicalExpression::GT(left_box, right_box),
            BinaryOperator::LT(_) => ArithmeticOrLogicalExpression::LT(left_box, right_box),
            BinaryOperator::GTE(_) => ArithmeticOrLogicalExpression::GTE(left_box, right_box),
            BinaryOperator::LTE(_) => ArithmeticOrLogicalExpression::LTE(left_box, right_box),
            BinaryOperator::EQ(_) => ArithmeticOrLogicalExpression::EQ(left_box, right_box),
            BinaryOperator::NEQ(_) => ArithmeticOrLogicalExpression::NEQ(left_box, right_box),
            BinaryOperator::Or => ArithmeticOrLogicalExpression::Or(left_box, right_box),
            BinaryOperator::And => ArithmeticOrLogicalExpression::And(left_box, right_box),
            BinaryOperator::Xor => ArithmeticOrLogicalExpression::Xor(left_box, right_box),
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
            BinaryOperator::Or => Precedence::Or,
            BinaryOperator::And => Precedence::And,
            BinaryOperator::Xor => Precedence::Xor,
        }
    }
}

impl ArithmeticOrLogicalExpression {
    pub fn parse_right_expression<'a>(
        input: &'a ParseBuffer<'a>,
        operator_precedence: Precedence,
    ) -> Result<Expression, Error> {
        let result = if input.peek(Paren) {
            Expression::parse_item(input)
        } else if UnaryOperator::peek(input) {
            let unary_operator: UnaryOperator = input.parse()?;
            let right = Self::parse_right_expression(input, unary_operator.precedence())?;
            Ok(Expression::ArithmeticOrLogicalExpression(
                unary_operator.construct_expr(right),
            ))
        } else if Literal::peek(input){
            let literal = input.parse()?;
            Ok(Expression::Literal(literal))
        } else if DatabaseIdent::peek(input) {
            let ident = input.parse()?;
            Ok(Expression::Ident(ident))
        } else {
            Err(input.error("Cannot parse into expression or unary operator"))
        }?;

        let next_binary_operator_precedence = Precedence::peek(input);

        match next_binary_operator_precedence {
            Some(next_precedence) if next_precedence > operator_precedence => {
                let operator = input.parse::<BinaryOperator>()?;
                let next_expr = Self::parse_right_expression(input, operator.precedence())?;

                Ok(Expression::ArithmeticOrLogicalExpression(
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
            let operator = if BinaryOperator::peek(input) {
                input.parse::<BinaryOperator>()?
            } else {
                break;
            };

            result = Expression::ArithmeticOrLogicalExpression(operator.construct_expr(
                result,
                Self::parse_right_expression(input, operator.precedence())?,
            ))
        }

        Ok(result)
    }
}

impl Parse for ArithmeticOrLogicalExpression {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let left = Self::parse_right_expression(input, Precedence::None)?;

        match Self::parse_operator_and_right_expression(input, left)? {
            Expression::ArithmeticOrLogicalExpression(mathematical_expr) => Ok(mathematical_expr),
            _ => Err(input.error("Not a mathematical expression")),
        }
    }
}

impl Peekable for ArithmeticOrLogicalExpression {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        Precedence::peek(input).is_some()
    }
}


