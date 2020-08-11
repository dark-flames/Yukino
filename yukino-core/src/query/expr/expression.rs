use crate::query::expr::mathematical::MathematicalExpression;
use crate::query::{
    BinaryOperator, Function, IdentExpression, Peekable, SubqueryExpression, UnaryOperator, Value,
};
use proc_macro2::Span;
use syn::parse::{Parse, ParseBuffer};
use syn::token::Paren;
use syn::{parenthesized, Error};

pub enum ExpressionStructure {
    MathematicalExpr(MathematicalExpression),
    SubqueryExpr(SubqueryExpression),
    IdentExpr(IdentExpression),
    Function(Function),
    Value(Value),
}

impl Parse for ExpressionStructure {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let mut result = Self::parse_item(input)?;

        while !input.is_empty() {
            if BinaryOperator::peek(input) {
                result =
                    MathematicalExpression::parse_operator_and_right_expression(input, result)?;
            } else if SubqueryExpression::peek_in(input) {
                result = SubqueryExpression::parse_right_and_operator(input, result)?;
            } else {
                return Err(Error::new(Span::call_site(), "Unexpected expression part"));
            }
        }

        Ok(result)
    }
}

#[allow(dead_code)]
impl ExpressionStructure {
    pub fn parse_item<'a>(input: &'a ParseBuffer<'a>) -> Result<ExpressionStructure, Error> {
        if input.peek(Paren) {
            let content;

            parenthesized!(content in input);

            content.parse()
        } else if UnaryOperator::peek(input) {
            input.parse().map(ExpressionStructure::MathematicalExpr)
        } else if Function::peek(input) {
            input.parse().map(ExpressionStructure::Function)
        } else if SubqueryExpression::peek(input) {
            input.parse().map(ExpressionStructure::SubqueryExpr)
        } else if IdentExpression::peek(input) {
            input.parse().map(ExpressionStructure::IdentExpr)
        } else if Value::peek(input) {
            input.parse().map(ExpressionStructure::Value)
        } else {
            Err(Error::new(Span::call_site(), "Unexpected expression item"))
        }
    }
}
