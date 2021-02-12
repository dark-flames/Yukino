use crate::query::helper::Peekable;
use syn::parse::{Parse, ParseBuffer};
use syn::{token::Paren, Error, Ident, Token};

#[derive(Eq, PartialEq, Debug)]
pub struct DatabaseIdent {
    segments: Vec<String>,
}

impl Parse for DatabaseIdent {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let mut segments = vec![];

        let ahead = input.lookahead1();

        if ahead.peek(Ident) {
            segments.push(input.parse::<Ident>()?.to_string())
        }

        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;

            segments.push(input.parse::<Ident>()?.to_string())
        }

        Ok(DatabaseIdent { segments })
    }
}

impl Peekable for DatabaseIdent {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Ident) && !input.peek2(Paren)
    }
}

#[test]
fn test_ident() {
    use syn::parse_quote;
    let ident: DatabaseIdent = parse_quote! {
        a.b.c
    };

    assert_eq!(
        ident,
        DatabaseIdent {
            segments: vec!["a".to_string(), "b".to_string(), "c".to_string()]
        }
    )
}
