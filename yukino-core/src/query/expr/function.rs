use crate::query::{Expression, Peekable};
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseBuffer};
use syn::{parenthesized, Error, token::Paren, Ident as IdentMark};

pub enum Function {
    Average(Box<Expression>),
    Max(Box<Expression>),
    Min(Box<Expression>),
    Count(Box<Expression>),
    Distinct(Box<Expression>),
    Abs(Box<Expression>),
    Contact(Vec<Expression>),
}

impl Parse for Function {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let ident: Ident = input.parse::<Ident>()?;
        let content;
        parenthesized!(content in input);
        let expr = Box::new(content.parse::<Expression>()?);
        match ident.to_string().to_lowercase().as_str() {
            "average" => Ok(Function::Average(expr)),
            "max" => Ok(Function::Max(expr)),
            "min" => Ok(Function::Min(expr)),
            "count" => Ok(Function::Count(expr)),
            "distinct" => Ok(Function::Distinct(expr)),
            "abs" => Ok(Function::Abs(expr)),
            "contact" => Ok(Function::Abs(expr)),
            _ => Err(Error::new(Span::call_site(), "Unexpected function")),
        }
    }
}

impl Peekable for Function {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(IdentMark) && input.peek2(Paren)
    }
}
