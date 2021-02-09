use crate::query::expr::helper::Peekable;
use crate::query::expr::Expression;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Ident as IdentMark, Token};

pub enum UnaryExpression {
    Not(Box<Expression>),
    BitInverse(Box<Expression>),
}

#[allow(dead_code)]
enum UnaryOperator {
    Not,
    BitInverse(Token![~]),
}

#[allow(dead_code)]
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
