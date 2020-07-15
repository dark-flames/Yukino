use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Token, token::Comma, Expr};
use proc_macro2::Ident;

#[derive(Eq, PartialEq, Debug)]
pub struct EntityField {
    pub class: Ident,
    pub field: Ident
}

impl Parse for EntityField {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let class = input.parse()?;
        input.parse::<Token![::]>()?;
        let field = input.parse()?;

        Ok(EntityField {
            class,
            field
        })
    }
}

pub struct FieldAssignment {
    pub field: EntityField,
    pub value: Expr
}

impl Parse for FieldAssignment {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let field = input.parse()?;
        input.parse::<Token![=]>()?;

        let value = input.parse()?;

        Ok(FieldAssignment {
            field,
            value
        })
    }
}

pub struct FieldAssignments(pub Vec<FieldAssignment>);

impl Parse for FieldAssignments {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let mut result = Vec::new();

        loop {
            let item = input.parse::<FieldAssignment>()?;

            result.push(item);

            if input.parse::<Comma>().is_err() {
                break;
            }
        }


        Ok(FieldAssignments(result))
    }
}

#[test]
fn test_field_assignment() {
    use syn::{export::ToTokens, parse_quote};
    use proc_macro2::Span;
    let result: FieldAssignments = parse_quote! {
        Foo::bar = test, Bar::foo = test2
    };

    assert_eq!(result.0[0].field, EntityField {
        class: Ident::new("Foo", Span::call_site()),
        field: Ident::new("bar", Span::call_site())
    });
    assert_eq!(result.0[1].field, EntityField {
        class: Ident::new("Bar", Span::call_site()),
        field: Ident::new("foo", Span::call_site())
    });

    assert_eq!(result.0[0].value.to_token_stream().to_string(), "test".to_string());
    assert_eq!(result.0[1].value.to_token_stream().to_string(), "test2".to_string());
}

