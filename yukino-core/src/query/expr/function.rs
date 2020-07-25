use crate::query::Expression;
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseBuffer};
use syn::Error;

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
        let expr = Box::new(input.parse::<Expression>()?);
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
