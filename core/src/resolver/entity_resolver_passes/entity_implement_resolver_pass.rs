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
        let proxy_ident = format_ident!("{}", entity_name);

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
                        map.extend(Self::#method().primary_key_values_by_ref(&self.#field_ident)?)
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

        Some(Ok(quote! {
            impl<'t> yukino::Entity<'t> for #ident<'t> {
                type Proxy: EntityProxy<'t, Self> = #proxy_ident<'t>;
                fn from_database_value(
                    result: &std::collections::HashMap<String, yukino::types::DatabaseValue>
                ) -> Result<Self, yukino::resolver::error::DataConvertError>
                where Self: Sized{
                    use yukino::resolver::ValueConverter;

                    #(#temp_values;)*

                    Ok(#ident {
                        #(#fields,)*
                        _repository_life_time_marker: Default::default()
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
