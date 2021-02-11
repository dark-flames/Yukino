use crate::query::expr::expression::Expression;
use crate::query::expr::helper::Peekable;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseBuffer};
use syn::{parenthesized, token::Paren, Error, Ident as IdentMark, Token};

#[derive(Debug, Eq, PartialEq)]
pub struct FunctionCall {
    ident: Ident,
    parameters: Vec<Expression>,
}

impl Parse for FunctionCall {
    fn parse(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let ident: Ident = input.parse()?;
        let mut parameters: Vec<Expression> = vec![];
        let parameter_content;
        parenthesized!(parameter_content in input);
        loop {
            parameters.push(parameter_content.parse()?);

            if parameter_content.peek(Token![,]) {
                parameter_content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(FunctionCall { ident, parameters })
    }
}

impl Peekable for FunctionCall {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(IdentMark) && input.peek2(Paren)
    }
}
