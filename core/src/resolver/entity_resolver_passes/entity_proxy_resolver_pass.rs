use crate::definitions::TableDefinition;
use crate::resolver::error::ResolveError;
use crate::resolver::{
    AchievedFieldResolver, EntityResolverPass, EntityResolverPassBox, FieldName, TypePathResolver,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
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
        let inner_ident = format_ident!("{}Inner", &entity_name);

        let visitors = field_resolvers
            .iter()
            .fold(vec![], |mut list, (_, resolver)| {
                list.push(&resolver.field_getter_token_stream);
                list.push(&resolver.field_setter_token_stream);
                list
            });

        let params: Vec<_> = field_resolvers
            .iter()
            .map(|(_, resolver)| {
                let field_ident = format_ident!("{}", resolver.field_path.1);
                let field_type = &resolver.field_type;
                quote! {
                    #field_ident: #field_type
                }
            })
            .collect();

        let field_idents: Vec<_> = field_resolvers
            .iter()
            .map(|(_, resolver)| format_ident!("{}", resolver.field_path.1))
            .collect();

        let create = quote! {
            pub fn with_value(
                #(#params,)*
            ) -> impl FnOnce() -> #inner_ident<'t> {
                move || {
                    #inner_ident {
                        #(#field_idents,)*
                        _repository_life_time_marker: Default::default()
                    }
                }
            }
        };

        Some(Ok(quote! {
            pub struct #ident<'t> {
                inner: #inner_ident<'t>,
                unique_id: Option<yukino::EntityUniqueID>,
                repository: &'t yukino::repository::Repository<'t, #inner_ident<'t>>,
            }

            impl<'t> yukino::EntityProxy<'t, #inner_ident<'t>> for #ident<'t> {
                fn unique_id(&self) -> Option<yukino::EntityUniqueID> {
                    self.unique_id.clone()
                }

                fn set_unique_id(&mut self, unique_id: yukino::EntityUniqueID) {
                    self.unique_id = Some(unique_id);
                }

                fn get_repository(
                    &self,
                ) -> &'t yukino::repository::Repository<'t, #inner_ident<'t>>
                    where
                        Self: Sized {
                    self.repository
                }

                fn create_proxy(
                    inner: #inner_ident<'t>,
                    repository: &'t yukino::repository::Repository<'t, #inner_ident<'t>>,
                ) -> Self
                where
                    Self: Sized,
                {
                    #ident {
                        inner,
                        unique_id: None,
                        repository,
                    }
                }

                fn inner(&self) -> #inner_ident<'t> {
                    self.inner.clone()
                }
            }

            impl<'t> #ident<'t> {
                #(#visitors)*

                #create
            }

            impl<'t> Drop for #ident<'t> {
                fn drop(&mut self) {
                    use yukino::EntityProxy;
                    self.drop_from_pool()
                }
            }
        }))
    }
}
