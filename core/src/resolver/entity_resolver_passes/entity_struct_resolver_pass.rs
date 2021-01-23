use crate::definitions::TableDefinition;
use crate::resolver::error::ResolveError;
use crate::resolver::{
    AchievedFieldResolver, EntityResolverPass, EntityResolverPassBox, FieldName, TypePathResolver,
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{
    parse_quote, Field, Fields, GenericParam, ItemStruct, LifetimeDef, Token, Type, Visibility,
};

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
            if matches!(field.vis, Visibility::Inherited) {
                return Some(Err(ResolveError::FieldVisibilityMustBePrivate(
                    entity_name,
                    field.ident.as_ref().unwrap().to_string(),
                )));
            }

            if let Type::Path(type_path) = &field.ty {
                field.ty = Type::Path(type_path_resolver.get_full_path(type_path.clone()))
            }
        }

        struct_item
            .generics
            .params
            .push(GenericParam::Lifetime(LifetimeDef::new(parse_quote! {'t})));

        if let Fields::Named(named_filed) = &mut struct_item.fields {
            named_filed.named.push(Field {
                attrs: vec![],
                vis: Visibility::Inherited,
                ident: Some(format_ident!("_repository_life_time_marker")),
                colon_token: Some(Token![:](Span::mixed_site())),
                ty: parse_quote! {
                    std::marker::PhantomData<&'t ()>
                },
            });
        }

        Some(Ok(quote! {
        #[derive(Clone)]
            #struct_item

            impl<'t> #new_ident<'t> {
                #(#converters)*
            }
        }))
    }
}
