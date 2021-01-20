use crate::definitions::TableDefinition;
use crate::resolver::{AchievedFieldResolver, EntityResolverPass, FieldName};
use proc_macro2::{Ident, TokenStream};
use std::collections::HashMap;

pub struct ConverterGetterResolverPass;

impl EntityResolverPass for ConverterGetterResolverPass {
    fn new() -> Self
    where
        Self: Sized,
    {
        ConverterGetterResolverPass
    }

    fn boxed(&self) -> Box<dyn EntityResolverPass> {
        Box::new(ConverterGetterResolverPass)
    }

    fn get_methods_token_stream(
        &self,
        _entity_path: String,
        _ident: &Ident,
        _definitions: &[TableDefinition],
        field_resolvers: &HashMap<FieldName, AchievedFieldResolver>,
    ) -> Option<Vec<TokenStream>> {
        Some(
            field_resolvers
                .values()
                .into_iter()
                .map(|resolver| resolver.data_converter_token_stream.clone())
                .collect(),
        )
    }
}
