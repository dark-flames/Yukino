use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Ident, Token};

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug)]
pub struct IdentExpression {
    segments: Vec<String>,
}

impl Parse for IdentExpression {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let mut segments = vec![];

        let ahead = input.lookahead1();

        if ahead.peek(Ident) {
            segments.push(
                input.parse::<Ident>()?.to_string()
            )
        }

        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;

            segments.push(
                input.parse::<Ident>()?.to_string()
            )
        }

        Ok(IdentExpression {
            segments
        })
    }
}

#[test]
fn test_ident() {
    use syn::parse_quote;
    let ident: IdentExpression = parse_quote! {
        a.b.c
    };

    assert_eq!(ident , IdentExpression { segments: vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string()
    ] })
}
