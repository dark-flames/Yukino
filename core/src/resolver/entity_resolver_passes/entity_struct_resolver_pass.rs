use crate::definitions::TableDefinition;
use crate::resolver::error::ResolveError;
use crate::resolver::{
    AchievedFieldResolver, EntityResolverPass, EntityResolverPassBox, FieldName, TypePathResolver,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{ItemStruct, Type, Visibility};

pub struct EntityStructResolverPass;

impl EntityResolverPass for EntityStructResolverPass {
    fn new() -> Self
    where
        Self: Sized,
    {
        EntityStructResolverPass
    }

    fn boxed(&self) -> EntityResolverPassBox {
        Box::new(EntityStructResolverPass)
    }

    fn get_implement_token_stream(
        &self,
        entity_name: String,
        _definitions: &[TableDefinition],
        field_resolvers: &HashMap<FieldName, AchievedFieldResolver>,
        input: &ItemStruct,
        type_path_resolver: &TypePathResolver,
    ) -> Option<Result<TokenStream, ResolveError>> {
        let new_ident = format_ident!("{}Inner", entity_name);

        let converters: Vec<_> = field_resolvers
            .values()
            .into_iter()
            .map(|resolver| resolver.data_converter_token_stream.clone())
            .collect();

        let mut struct_item = input.clone();

        struct_item.ident = new_ident.clone();
        struct_item.attrs = vec![];

        if !struct_item.generics.params.is_empty() {
            return Some(Err(ResolveError::GenericIsNotSupported(entity_name)));
        } else if !matches!(&struct_item.vis, Visibility::Public(_)) {
            return Some(Err(ResolveError::EntityVisibilityMustBePublic(entity_name)));
        }

        for field in struct_item.fields.iter_mut() {
            field.attrs = vec![];
            if !matches!(field.vis, Visibility::Inherited) {
                return Some(Err(ResolveError::FieldVisibilityMustBePrivate(
                    entity_name,
                    field.ident.as_ref().unwrap().to_string(),
                )));
            }

            let field_name = field.ident.as_ref().unwrap().to_string();

            let resolver = match field_resolvers.get(&field_name) {
                Some(r) => r,
                _ => {
                    return Some(Err(ResolveError::FieldResolverNotFound(
                        entity_name,
                        field_name,
                    )))
                }
            };

            if let Type::Path(type_path) = &resolver.field_type {
                field.ty = Type::Path(type_path_resolver.get_full_path(type_path.clone()))
            }
        }

        Some(Ok(quote! {
        #[derive(Clone)]
            #struct_item

            impl #new_ident {
                #(#converters)*
            }
        }))
    }
}
