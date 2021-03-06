use crate::definitions::TableDefinition;
use crate::resolver::error::ResolveError;
use crate::resolver::{
    AchievedFieldResolver, EntityResolverPass, EntityResolverPassBox, FieldName, TypePathResolver,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::ItemStruct;

pub struct EntityImplementResolverPass;

impl EntityResolverPass for EntityImplementResolverPass {
    fn new() -> Self
    where
        Self: Sized,
    {
        EntityImplementResolverPass
    }

    fn boxed(&self) -> EntityResolverPassBox {
        Box::new(EntityImplementResolverPass)
    }

    fn get_implement_token_stream(
        &self,
        entity_name: String,
        definitions: &[TableDefinition],
        field_resolvers: &HashMap<FieldName, AchievedFieldResolver>,
        _input: &ItemStruct,
        _type_path_resolver: &TypePathResolver,
    ) -> Option<Result<TokenStream, ResolveError>> {
        let ident = format_ident!("{}Inner", entity_name);

        let temp_values: Vec<_> = field_resolvers
            .values()
            .map(|resolver| {
                let method = resolver.converter_getter_ident.clone();

                let field_ident = format_ident!("{}", &resolver.field_path.1);

                quote::quote! {
                    let #field_ident = Self::#method().to_field_value(result)?
                }
            })
            .collect();

        let fields: Vec<_> = field_resolvers
            .values()
            .map(|resolver| format_ident!("{}", &resolver.field_path.1))
            .collect();

        let inserts: Vec<_> = field_resolvers
            .values()
            .map(|resolver| {
                let method = resolver.converter_getter_ident.clone();

                let field_ident = format_ident!("{}", &resolver.field_path.1);

                quote::quote! {
                    map.extend(Self::#method().to_database_values_by_ref(&self.#field_ident)?)
                }
            })
            .collect();

        let primary_key_inserts: Vec<_> = field_resolvers
            .values()
            .filter_map(|resolver| {
                if !resolver.primary_key_column_names().is_empty() {
                    let method = resolver.converter_getter_ident.clone();

                    let field_ident = format_ident!("{}", &resolver.field_path.1);

                    Some(quote::quote! {
                        map.extend(Self::#method().primary_column_values_by_ref(&self.#field_ident)?)
                    })
                } else {
                    None
                }
            })
            .collect();

        let (mut_token, use_tokens) = if primary_key_inserts.is_empty() {
            (TokenStream::new(), TokenStream::new())
        } else {
            (
                quote::quote! {mut},
                quote::quote! {use yukino::resolver::ValueConverter},
            )
        };

        let field_definitions: Vec<_> = field_resolvers
            .iter()
            .map(|(_, resolver)| {
                let definition = resolver.field_definition.clone();
                let name = definition.name.as_str();

                quote::quote! {
                    #name => Some(#definition)
                }
            })
            .collect();

        Some(Ok(quote! {
            impl yukino::Entity for #ident {
                fn from_database_value(
                    result: &std::collections::HashMap<String, yukino::types::DatabaseValue>
                ) -> Result<Self, yukino::resolver::error::DataConvertError>
                where Self: Sized{
                    use yukino::resolver::ValueConverter;

                    #(#temp_values;)*

                    Ok(#ident {
                        #(#fields),*
                    })
                }

                fn to_database_values(&self)
                    -> Result<
                        std::collections::HashMap<String, yukino::types::DatabaseValue>,
                        yukino::resolver::error::DataConvertError
                    > {
                    let mut map = std::collections::HashMap::new();
                    use yukino::resolver::ValueConverter;
                    #(#inserts;)*

                    Ok(map)
                }

                fn get_definitions() -> Vec<yukino::definitions::TableDefinition> {
                    vec![
                        #(#definitions),*
                    ]
                }

                fn get_field_definition(field_name: &str) -> Option<yukino::definitions::FieldDefinition> {
                    match field_name {
                        #(#field_definitions,)*
                        _ => None
                    }
                }

                fn primary_key_values(&self) -> Result<
                        std::collections::HashMap<String, yukino::types::DatabaseValue>,
                        yukino::resolver::error::DataConvertError
                    > {
                    let #mut_token map = std::collections::HashMap::new();
                    #use_tokens;
                    #(#primary_key_inserts;)*

                    Ok(map)
                }
            }
        }))
    }
}
