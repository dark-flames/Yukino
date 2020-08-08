use crate::query::{Expression, IdentExpression, Peekable, Value};
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseBuffer};
use syn::{token::Paren, Error, Ident as IdentMark, Token};

pub enum LogicalExpression {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

pub enum BinaryLogicalOperator {
    And,
    Or,
    Xor,
}

impl Parse for BinaryLogicalOperator {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(IdentMark) {
            let ident: Ident = input.parse()?;
            let ident_str = ident.to_string().to_lowercase();
            match ident_str.as_str() {
                "and" => Ok(BinaryLogicalOperator::And),
                "or" => Ok(BinaryLogicalOperator::Or),
                "xor" => Ok(BinaryLogicalOperator::Xor),
                _ => Err(Error::new(
                    Span::call_site(),
                    "Unexpected binary logical operator",
                )),
            }
        } else if input.peek(Token![||]) {
            Ok(BinaryLogicalOperator::Or)
        } else if input.peek(Token![&&]) {
            Ok(BinaryLogicalOperator::And)
        } else {
            Err(Error::new(
                Span::call_site(),
                "Unexpected binary logical operator",
            ))
        }
    }
}

impl Peekable for BinaryLogicalOperator {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        match input.fork().parse::<Ident>() {
            Ok(ident) => match ident.to_string().to_lowercase().as_str() {
                "and" | "or" | "xor" => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl BinaryLogicalOperator {
    pub fn precedence(&self) -> LogicalPrecedence {
        match self {
            BinaryLogicalOperator::Or => LogicalPrecedence::Or,
            BinaryLogicalOperator::And => LogicalPrecedence::And,
            BinaryLogicalOperator::Xor => LogicalPrecedence::Xor,
        }
    }

    pub fn construct_expr(
        &self,
        expr_left: Expression,
        expr_right: Expression,
    ) -> LogicalExpression {
        match self {
            BinaryLogicalOperator::Or => {
                LogicalExpression::Or(Box::new(expr_left), Box::new(expr_right))
            }
            BinaryLogicalOperator::And => {
                LogicalExpression::And(Box::new(expr_left), Box::new(expr_right))
            }
            BinaryLogicalOperator::Xor => {
                LogicalExpression::Xor(Box::new(expr_left), Box::new(expr_right))
            }
        }
    }
}

pub enum UnaryLogicalOperator {
    Not,
}

impl Parse for UnaryLogicalOperator {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let ident: Ident = input.parse()?;
        let ident_str = ident.to_string().to_lowercase();
        match ident_str.as_str() {
            "not" => Ok(UnaryLogicalOperator::Not),
            _ => Err(Error::new(
                Span::call_site(),
                "Unexpected unary logical operator",
            )),
        }
    }
}

impl UnaryLogicalOperator {
    pub fn precedence(&self) -> LogicalPrecedence {
        match self {
            UnaryLogicalOperator::Not => LogicalPrecedence::Not,
        }
    }

    pub fn construct_expr(&self, expr: Expression) -> LogicalExpression {
        match self {
            UnaryLogicalOperator::Not => LogicalExpression::Not(Box::new(expr)),
        }
    }
}

impl Peekable for UnaryLogicalOperator {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        let fork = input.fork();
        if fork.peek(IdentMark) {}
        match fork.parse::<IdentMark>() {
            Ok(ident) => match ident.to_string().to_lowercase().as_str() {
                "not" => true,
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum LogicalPrecedence {
    Or,
    Xor,
    And,
    Not,
}

impl LogicalPrecedence {
    pub fn peek<'a>(input: &'a ParseBuffer<'a>) -> Option<Self> {
        if input.peek(IdentMark) {
            let ident: Ident = input.fork().parse().ok()?;
            let ident_str = ident.to_string().to_lowercase();
            match ident_str.as_str() {
                "or" => Some(LogicalPrecedence::Or),
                "xor" => Some(LogicalPrecedence::Xor),
                "and" => Some(LogicalPrecedence::And),
                "not" => Some(LogicalPrecedence::Not),
                _ => None,
            }
        } else if input.peek(Token![||]) {
            Some(LogicalPrecedence::Or)
        } else if input.peek(Token![&&]) {
            Some(LogicalPrecedence::And)
        } else {
            None
        }
    }
}

impl LogicalExpression {
    pub fn parse_right_expression<'a>(
        input: &'a ParseBuffer<'a>,
        operator_precedence: LogicalPrecedence,
    ) -> Result<Expression, Error> {
        let result = if input.peek(Paren) {
            Expression::parse_item(input)
        } else if let Ok(unary_operator) = input.parse::<UnaryLogicalOperator>() {
            let right =
                LogicalExpression::parse_right_expression(input, unary_operator.precedence())?;

            Ok(Expression::LogicalExpr(
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

        let next_binary_operator_precedence = LogicalPrecedence::peek(input);

        match next_binary_operator_precedence {
            Some(next_precedence) if next_precedence > operator_precedence => {
                let operator = input.parse::<BinaryLogicalOperator>()?;
                let next_expr = Self::parse_right_expression(input, operator.precedence())?;

                Ok(Expression::LogicalExpr(
                    operator.construct_expr(result, next_expr),
                ))
            }
            _ => Ok(result),
        }
    }
}
