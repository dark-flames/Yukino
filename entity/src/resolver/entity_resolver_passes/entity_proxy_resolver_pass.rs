use crate::definitions::TableDefinition;
use crate::resolver::error::ResolveError;
use crate::resolver::{
    AchievedFieldResolver, EntityResolverPass, EntityResolverPassBox, FieldName, TypePathResolver,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::str::FromStr;
use syn::ItemStruct;

pub struct EntityProxyResolverPass;

impl EntityResolverPass for EntityProxyResolverPass {
    fn new() -> Self
    where
        Self: Sized,
    {
        EntityProxyResolverPass
    }

    fn boxed(&self) -> EntityResolverPassBox {
        Box::new(EntityProxyResolverPass)
    }

    fn get_implement_token_stream(
        &self,
        entity_name: String,
        _definitions: &[TableDefinition],
        field_resolvers: &HashMap<FieldName, AchievedFieldResolver>,
        _input: &ItemStruct,
        _type_path_resolver: &TypePathResolver,
    ) -> Option<Result<TokenStream, ResolveError>> {
        let ident = TokenStream::from_str(&entity_name).unwrap();

        let visitors = field_resolvers
            .iter()
            .fold(vec![], |mut list, (_, resolver)| {
                list.push(&resolver.field_getter_token_stream);
                list.push(&resolver.field_setter_token_stream);
                list
            });

        Some(Ok(quote! {
            yukino::impl_entity_proxy!(#ident);

            impl<'r> #ident<'r> {
                #(#visitors)*
            }
        }))
    }
}
