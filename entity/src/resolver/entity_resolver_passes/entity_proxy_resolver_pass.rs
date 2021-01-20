use crate::definitions::TableDefinition;
use crate::resolver::{AchievedFieldResolver, EntityResolverPass, FieldName};
use proc_macro2::{Ident, TokenStream};
use std::collections::HashMap;
use syn::DeriveInput;

pub struct EntityProxyResolverPass;

impl EntityResolverPass for EntityProxyResolverPass {
    fn new() -> Self
    where
        Self: Sized,
    {
        EntityProxyResolverPass
    }

    fn boxed(&self) -> Box<dyn EntityResolverPass> {
        Box::new(EntityProxyResolverPass)
    }

    fn get_implement_token_stream(
        &self,
        _entity_path: String,
        _ident: &Ident,
        _definitions: &[TableDefinition],
        _field_resolvers: &HashMap<FieldName, AchievedFieldResolver>,
        _derive_input: &DeriveInput,
    ) -> Option<TokenStream> {
        unimplemented!()
    }
}
