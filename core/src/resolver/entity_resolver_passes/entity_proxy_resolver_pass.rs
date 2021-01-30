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
            ) -> impl FnOnce() -> #inner_ident {
                move || {
                    #inner_ident {
                        #(#field_idents),*
                    }
                }
            }
        };

        Some(Ok(quote! {
            pub struct #ident<'t> {
                inner: std::cell::UnsafeCell<#inner_ident>,
                unique_id: Option<yukino::EntityUniqueID>,
                transaction: &'t yukino::Transaction,
            }

            impl<'t> yukino::EntityProxy<'t, #inner_ident> for #ident<'t> {
                fn unique_id(&self) -> Option<yukino::EntityUniqueID> {
                    self.unique_id.clone()
                }

                fn set_unique_id(&mut self, unique_id: yukino::EntityUniqueID) {
                    self.unique_id = Some(unique_id);
                }

                fn get_transaction(
                    &self,
                ) -> &'t yukino::Transaction
                    where
                        Self: Sized {
                    self.transaction
                }

                fn create_proxy(
                    inner: #inner_ident,
                    transaction: &'t yukino::Transaction,
                ) -> Self
                where
                    Self: Sized,
                {
                    #ident {
                        inner: std::cell::UnsafeCell::new(inner),
                        unique_id: None,
                        transaction,
                    }
                }

                fn inner(&self) -> #inner_ident {
                    self.get_inner().clone()
                }
            }

            impl<'t> #ident<'t> {
                #(#visitors)*

                #create

                fn get_inner(&self) -> & #inner_ident {
                    unsafe {
                        self.inner.get() as &#inner_ident
                    }
                }

                fn get_inner_mut(&self) -> &mut #inner_ident {
                    unsafe {
                        self.inner.get() as &mut #inner_ident
                    }
                }
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
