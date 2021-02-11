use crate::query::expr::function::FunctionCall;
use crate::query::expr::helper::Peekable;
use crate::query::expr::ident::DatabaseIdent;
use crate::query::expr::literal::Literal;
use crate::query::expr::mathematical::{
    ArithmeticOrLogicalExpression, BinaryOperator, UnaryOperator,
};
use syn::parse::{Parse, ParseBuffer};
use syn::{parenthesized, token::Paren, Error};

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Ident(DatabaseIdent),
    Function(FunctionCall),
    ArithmeticOrLogicalExpression(ArithmeticOrLogicalExpression),
}

impl Parse for Expression {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let mut result = Self::parse_item(input)?;

        while BinaryOperator::peek(input) {
            result =
                ArithmeticOrLogicalExpression::parse_operator_and_right_expression(input, result)?;
        }

        Ok(result)
    }
}

impl Expression {
    pub fn parse_item<'a>(input: &'a ParseBuffer<'a>) -> Result<Expression, Error> {
        if input.peek(Paren) {
            let content;

            parenthesized!(content in input);

            content.parse()
        } else if UnaryOperator::peek(input) {
            input.parse().map(Expression::ArithmeticOrLogicalExpression)
        } else if FunctionCall::peek(input) {
            input.parse().map(Expression::Function)
        } else if DatabaseIdent::peek(input) {
            input.parse().map(Expression::Ident)
        } else if Literal::peek(input) {
            input.parse().map(Expression::Literal)
        } else {
            Err(input.error("Unexpected expression item"))
        }
    }
}

#[test]
fn test_expr_1() {
    use syn::parse_quote;
    let result: Expression = parse_quote! {
        @value + 10 * test.t == Sum(1, test.f) * 10 + 1 or true
    };

    let result2: Expression = parse_quote! {
        ((@value + (10 * test.t)) == ((Sum(1, test.f) * 10) + 1)) or true
    };

    assert_eq!(result, result2);
}
