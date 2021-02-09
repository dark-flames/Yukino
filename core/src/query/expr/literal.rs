use crate::query::expr::expression::TypeFlag;
use crate::query::expr::helper::Peekable;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Lit, Token};

#[derive(Eq, PartialEq, Debug)]
pub enum Literal {
    Immediate { content: Lit, ty: TypeFlag },
    External { ident: Ident, ty: TypeFlag },
}

impl Parse for Literal {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let ahead = input.lookahead1();
        if ahead.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Ok(Literal::External {
                ident: input.parse()?,
                ty: TypeFlag::Unresolved,
            })
        } else if ahead.peek(Lit) {
            Ok(Literal::Immediate {
                content: input.parse()?,
                ty: TypeFlag::default(),
            })
        } else {
            Err(input.error("Cannot parse into an literal"))
        }
    }
}

impl Peekable for Literal {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Lit) || input.peek(Token![#])
    }
}

#[test]
fn test_value() {
    use syn::parse_quote;

    let value_lit: Literal = parse_quote! {
        "foo"
    };

    if let Literal::Immediate {
        content: Lit::Str(lit),
        ty: _,
    } = value_lit
    {
        assert_eq!(lit.value(), "foo".to_string())
    } else {
        panic!();
    };

    let value_external: Literal = parse_quote! {
        @bar
    };

    if let Literal::External { ident, ty: _ } = value_external {
        assert_eq!(ident.to_string(), "bar".to_string())
    } else {
        panic!();
    }
}
