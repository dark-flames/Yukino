use crate::query::helper::Peekable;
use float_eq::float_eq;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Lit, Token};

#[derive(Debug)]
pub enum Literal {
    Bool(bool),
    Int(usize),
    Float(f64),
    Str(String),
    Char(char),
    External(String),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Literal::Bool(x) => matches!(other, Literal::Bool(y) if x== y),
            Literal::Int(x) => matches!(other, Literal::Int(y) if x== y),
            Literal::Str(x) => matches!(other, Literal::Str(y) if x== y),
            Literal::Char(x) => matches!(other, Literal::Char(y) if x== y),
            Literal::External(x) => matches!(other, Literal::External(y) if x== y),
            Literal::Float(x) => matches!(other, Literal::Float(y) if float_eq!(x, y, ulps <= 4)),
        }
    }
}

impl Eq for Literal {}

impl Parse for Literal {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            let ident: Ident = input.parse()?;
            Ok(Literal::External(ident.to_string()))
        } else if input.peek(Lit) {
            match input.parse::<Lit>()? {
                Lit::Str(lit_str) => Ok(Literal::Str(lit_str.value())),
                Lit::Char(lit_char) => Ok(Literal::Char(lit_char.value())),
                Lit::Bool(lit_bool) => Ok(Literal::Bool(lit_bool.value)),
                Lit::Int(lit_int) => Ok(Literal::Int(lit_int.base10_parse()?)),
                Lit::Float(lit_float) => Ok(Literal::Float(lit_float.base10_parse()?)),
                _ => Err(input.error("Cannot parse into an literal")),
            }
        } else {
            Err(input.error("Cannot parse into an literal"))
        }
    }
}

impl Peekable for Literal {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Lit) || input.peek(Token![@])
    }
}

#[test]
fn test_value() {
    use syn::parse_quote;

    let value_lit: Literal = parse_quote! {
        "foo"
    };

    if let Literal::Str(lit) = value_lit {
        assert_eq!(lit, "foo".to_string())
    } else {
        panic!();
    };

    let value_external: Literal = parse_quote! {
        @bar
    };

    if let Literal::External(ident) = value_external {
        assert_eq!(ident.to_string(), "bar".to_string())
    } else {
        panic!();
    }
}
