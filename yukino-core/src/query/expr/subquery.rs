use crate::query::{Expression, Peekable, Value};
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseBuffer};
use syn::{parenthesized, token::Paren, Error, Ident as IdentMark};

pub enum SubqueryExpression {
    In(Box<Expression>, Box<Value>),
    Any(Box<Value>),
    Some(Box<Value>),
    ALL(Box<Value>),
    Exists(Box<Value>),
}

impl Parse for SubqueryExpression {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let ident: Ident = input.parse::<Ident>()?;
        let content;
        parenthesized!(content in input);
        let expr = Box::new(content.parse()?);
        match ident.to_string().to_lowercase().as_str() {
            "any" => Ok(SubqueryExpression::Any(expr)),
            "some" => Ok(SubqueryExpression::Some(expr)),
            "all" => Ok(SubqueryExpression::ALL(expr)),
            "exists" => Ok(SubqueryExpression::Exists(expr)),
            _ => Err(Error::new(Span::call_site(), "Unexpected function")),
        }
    }
}

impl Peekable for SubqueryExpression {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(IdentMark)
            && input.peek2(Paren)
            && match input.fork().parse::<Ident>() {
                Ok(ident) => match ident.to_string().as_str() {
                    "any" | "some" | "all" | "exists" => true,
                    _ => false,
                },
                _ => false,
            }
    }
}

impl SubqueryExpression {
    pub fn peek_in<'a>(input: &'a ParseBuffer<'a>) -> bool {
        if input.peek(IdentMark) {
            match input.fork().parse::<Ident>() {
                Ok(ident) => ident == "in",
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn parse_right_and_operator<'a>(
        input: &'a ParseBuffer<'a>,
        left: Expression,
    ) -> Result<Expression, Error> {
        if !Self::peek_in(input) {
            return Err(Error::new(Span::call_site(), "Can not find operator 'IN'"));
        }

        input.parse::<Ident>()?;

        let value = input.parse::<Value>()?;

        Ok(Expression::SubqueryExpr(Self::In(
            Box::new(left),
            Box::new(value),
        )))
    }
}
