use proc_macro2::{Ident, TokenStream};
use syn::parse::{Parse, ParseBuffer};
use syn::{token::Comma, Error, Expr, Token};
use quote::{format_ident, quote};
use heck::SnakeCase;

#[derive(Eq, PartialEq, Debug)]
pub struct EntityField {
    pub class: Ident,
    pub field: Ident,
}

impl Parse for EntityField {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let class = input.parse()?;
        input.parse::<Token![::]>()?;
        let field = input.parse()?;

        Ok(EntityField { class, field })
    }
}

impl EntityField {
    pub fn get_convert(&self) -> TokenStream {
        let class = &self.class;
        let field_name = &self.field.to_string();
        let method = format_ident!("get_{}_converter", field_name.to_snake_case());

        quote! {
            #class::#method()
        }
    }
}

pub struct FieldAssignment {
    pub field: EntityField,
    pub value: Expr,
    pub is_ref: bool
}

impl Parse for FieldAssignment {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let field = input.parse()?;
        input.parse::<Token![=]>()?;

        let is_ref = input.parse::<Token![ref]>().is_ok();

        let value = input.parse()?;

        Ok(FieldAssignment { field, value, is_ref })
    }
}

impl FieldAssignment {
    pub fn to_assignment_items(&self) -> TokenStream {
        let converter = self.field.get_convert();
        let value = &self.value;

        let method = if self.is_ref {
            quote!{ to_database_value_by_ref }
        } else {
            quote!{ to_database_value }
        };

        quote! {
            #converter.#method(#value).map(
                |result| {
                    result.iter().map(
                        |(column, value)| {
                            yukino::query::AssignmentItem::new(column, value)
                        }
                    ).collect::<Vec<yukino::query::AssignmentItem>>()
                }
            )
        }
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

impl FieldAssignments {
    pub fn to_assignment_items(&self) -> TokenStream {
        let items = self.0.iter().map(
            |assignment| assignment.to_assignment_items()
        ).collect::<Vec<TokenStream>>();

        quote! {
            vec![
                #(#items),*
            ].iter()
            .flatten()
            .collect::<Vec<yukino::query::AssignmentItem>>()
        }
    }
}

#[test]
fn test_field_assignment() {
    use proc_macro2::Span;
    use syn::{export::ToTokens, parse_quote};
    let result: FieldAssignments = parse_quote! {
        Foo::bar = ref test, Bar::foo = test2
    };

    assert_eq!(
        result.0[0].field,
        EntityField {
            class: Ident::new("Foo", Span::call_site()),
            field: Ident::new("bar", Span::call_site())
        }
    );
    assert_eq!(
        result.0[1].field,
        EntityField {
            class: Ident::new("Bar", Span::call_site()),
            field: Ident::new("foo", Span::call_site())
        }
    );

    assert_eq!(
        result.0[0].value.to_token_stream().to_string(),
        "test".to_string()
    );
    assert_eq!(
        result.0[1].value.to_token_stream().to_string(),
        "test2".to_string()
    );

    assert_eq!(
        result.0[0].is_ref,
        true
    );
    assert_eq!(
        result.0[1].is_ref,
        false
    );
}
