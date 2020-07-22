use crate::query::{Expression, Value, IdentExpression};
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, token, parenthesized, Token};
use proc_macro2::Span;

pub enum MathematicalExpression {
    Paren(Box<Expression>),
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
enum BinaryOperator {
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

enum UnaryOperator {
    BitInverse(Token![~])
}

impl Parse for BinaryOperator {

    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![^])  {
            input.parse().map(BinaryOperator::BitXor)
        } else if input.peek(Token![*])  {
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
        } else if input.peek(Token![<])  {
            input.parse().map(BinaryOperator::LT)
        } else if input.peek(Token![>])  {
            input.parse().map(BinaryOperator::GT)
        } else if input.peek(Token![<=])  {
            input.parse().map(BinaryOperator::LTE)
        } else if input.peek(Token![>=])  {
            input.parse().map(BinaryOperator::GTE)
        } else if input.peek(Token![==])  {
            input.parse().map(BinaryOperator::EQ)
        } else if input.peek(Token![!=])  {
            input.parse().map(BinaryOperator::NEQ)
        } else {
            Err(Error::new(Span::call_site(), "Cannot parse into an binary operator"))
        }
    }
}

impl Parse for UnaryOperator {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![~]) {
            input.parse().map(UnaryOperator::BitInverse)
        }  else {
            Err(Error::new(Span::call_site(), "Cannot parse into an Unary operator"))
        }
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
            BinaryOperator::Sub(_)  => MathematicalExpression::Sub(left_box, right_box),
            BinaryOperator::BitLeftShift(_) => MathematicalExpression::BitLeftShift(left_box, right_box),
            BinaryOperator::BitRightShift(_) => MathematicalExpression::BitRightShift(left_box, right_box),
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
            BinaryOperator::Multi(_)
            | BinaryOperator::Div(_)
            | BinaryOperator::Mod(_) => Precedence::Term,
            BinaryOperator::Add(_)
            | BinaryOperator::Sub(_) => {
                Precedence::Add
            }
            BinaryOperator::BitLeftShift(_)
            | BinaryOperator::BitRightShift(_) => Precedence::BitShift,
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
            UnaryOperator::BitInverse(_) => MathematicalExpression::BitInverse(boxed_expr)
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            UnaryOperator::BitInverse(_) => Precedence::BitInverse
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
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> Option<Self>{
        if input.peek(Token![^])  {
            Some(Precedence::BitXor)
        } else if input.peek(Token![*])
            | input.peek(Token![/])
            | input.peek(Token![%]) {
            Some(Precedence::Term)
        } else if input.peek(Token![+])
            | input.peek(Token![-]) {
            Some(Precedence::Add)
        } else if input.peek(Token![>>])
            | input.peek(Token![<<]) {
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
            | input.peek(Token![!=]) {
            Some(Precedence::Comparison)
        } else {
            None
        }
    }
}


impl MathematicalExpression {
    pub fn parse_right_as_mathematical_expr<'a>(
        input: &'a ParseBuffer<'a>,
        operator_precedence: Precedence
    ) -> Result<Self, Error> {
        let expr = Self::parse_right_expression(input, operator_precedence)?;
        println!("finish {}", input.to_string());

        match expr {
            Expression::MathematicalExpr(mathematical_expr) => Ok(mathematical_expr),
            other_expr => Ok(MathematicalExpression::Paren(Box::new(other_expr)))
        }
    }

    pub fn parse_right_expression<'a>(
        input: &'a ParseBuffer<'a>,
        operator_precedence: Precedence
    ) -> Result<Expression, Error> {
        println!("parse {}", input.to_string());
        let result = if input.peek(token::Paren) {
            println!("from paren");
            Self::parse_from_parentheses(input).map(
                Expression::MathematicalExpr
            )
        } else if let Ok(unary_operator) = input.parse::<UnaryOperator>() {
            println!("from unary operator");
            let right = MathematicalExpression::parse_right_as_mathematical_expr(
                input,
                unary_operator.precedence()
            )?;

            Ok(Expression::MathematicalExpr(
                unary_operator.construct_expr(Expression::MathematicalExpr(right))
            ))
        } else if let Ok(value) = input.parse::<Value>() {
            println!("from value");
            Ok(Expression::Value(value))
        } else if let Ok(ident) = input.parse::<IdentExpression>() {
            println!("from ident: {:?}", ident);
            Ok(Expression::IdentExpr(ident))
        } else {
            Err(Error::new(Span::call_site(), "Cannot parse into expression or unary operator"))
        }?;

        println!("Finish first expr");

        let next_binary_operator_precedence = Precedence::peek(input);
        println!("Next operator {:?}", &next_binary_operator_precedence);

        match next_binary_operator_precedence {
            Some(next_precedence) if next_precedence > operator_precedence => {
                let operator = input.parse::<BinaryOperator>()?;
                println!("Continue merge");
                let next_expr = Self::parse_right_as_mathematical_expr(
                    input,
                    operator.precedence()
                )?;

                println!("start merge {:?}", &operator);

                Ok(Expression::MathematicalExpr(operator.construct_expr(
                    result,
                    Expression::MathematicalExpr(next_expr)
                )))
            },
            _ => Ok(result)
        }
    }

    fn parse_from_parentheses<'a>(
        input: &'a ParseBuffer<'a>,
    ) -> Result<Self, Error> {
        let content;
        parenthesized!(content in input);

        Ok(content.parse::<MathematicalExpression>()?)
    }
}

impl Parse for MathematicalExpression {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let mut result = Self::parse_right_expression(
            input,
            Precedence::None
        )?;

        while !input.is_empty() {
            println!("parse next part: {}", input);
            let operator = input.parse::<BinaryOperator>()?;

            println!("operator: {:?}", &operator);

            result = Expression::MathematicalExpr(operator.construct_expr(
                result,
                Self::parse_right_expression(
                    input,
                    operator.precedence()
                )?
            ))
        }

        match result {
            Expression::MathematicalExpr(mathematical_expr) => Ok(mathematical_expr),
            other_expr => Ok(MathematicalExpression::Paren(Box::new(other_expr)))
        }
    }
}

#[test]
fn test_mathematical_expr() {
    let expr: MathematicalExpression = syn::parse_quote! {
        1 + @value * 10 > ~10 % (5 + 3)
    };

    if let MathematicalExpression::GT(_, _) = expr {
        println!("OK")
    } else {
        panic!("GT")
    }
}
