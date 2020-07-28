use crate::query::Peekable;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Lit, Token};

/// ValueExpression
/// Lit: literal like string or number ...
/// ExternalValue: rust ident start with `@`
#[derive(Eq, PartialEq, Debug)]
pub enum Value {
    Lit(Lit),
    ExternalValue(Ident),
}

impl Parse for Value {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let ahead = input.lookahead1();
        if ahead.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Ok(Value::ExternalValue(input.parse()?))
        } else if ahead.peek(Lit) {
            Ok(Value::Lit(input.parse()?))
        } else {
            Err(ahead.error())
        }
    }
}

impl Peekable for Value {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Lit) || input.peek(Token![@])
    }
}

#[test]
fn test_value() {
    use syn::parse_quote;

    let value_lit: Value = parse_quote! {
        "foo"
    };

    if let Value::Lit(Lit::Str(lit)) = value_lit {
        assert_eq!(lit.value(), "foo".to_string())
    } else {
        panic!();
    };

    let value_external: Value = parse_quote! {
        @bar
    };

    if let Value::ExternalValue(ident) = value_external {
        assert_eq!(ident.to_string(), "bar".to_string())
    } else {
        panic!();
    }
}
