use crate::definitions::TableDefinition;
use crate::resolver::{AchievedFieldResolver, EntityResolverPass, FieldName, EntityResolverPassBox};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::str::FromStr;
use syn::DeriveInput;

pub struct ConverterGetterResolverPass;

impl EntityResolverPass for ConverterGetterResolverPass {
    fn new() -> Self
    where
        Self: Sized,
    {
        ConverterGetterResolverPass
    }

    fn boxed(&self) -> EntityResolverPassBox {
        Box::new(ConverterGetterResolverPass)
    }

    fn get_implement_token_stream(
        &self,
        entity_name: String,
        _ident: &Ident,
        _definitions: &[TableDefinition],
        field_resolvers: &HashMap<FieldName, AchievedFieldResolver>,
        _derive_input: &DeriveInput,
    ) -> Option<TokenStream> {
        let ident = TokenStream::from_str(&entity_name).unwrap();
        let converters: Vec<_> = field_resolvers
            .values()
            .into_iter()
            .map(|resolver| resolver.data_converter_token_stream.clone())
            .collect();

        Some(quote! {
            impl #ident {
                #(#converters)*
            }
        })
    }
}
