use crate::query::{ExpressionStructure, Peekable};
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseBuffer};
use syn::{parenthesized, token::Paren, Error, Ident as IdentMark};

pub enum Function {
    Average(Box<ExpressionStructure>),
    Max(Box<ExpressionStructure>),
    Min(Box<ExpressionStructure>),
    Count(Box<ExpressionStructure>),
    Distinct(Box<ExpressionStructure>),
    Abs(Box<ExpressionStructure>),
    Contact(Vec<ExpressionStructure>),
}

impl Parse for Function {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let ident: Ident = input.parse::<Ident>()?;
        let content;
        parenthesized!(content in input);
        let expr = Box::new(content.parse::<ExpressionStructure>()?);
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
        input.peek(IdentMark)
            && input.peek2(Paren)
            && match input.fork().parse::<Ident>() {
                Ok(ident) => match ident.to_string().as_str() {
                    "average" | "max" | "min" | "count" | "distinct" | "abs" | "contact" => true,
                    _ => false,
                },
                _ => false,
            }
    }
}
