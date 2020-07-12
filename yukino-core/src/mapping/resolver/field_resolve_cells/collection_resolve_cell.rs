use crate::mapping::definition::{ColumnDefinition, ForeignKeyDefinition, TableDefinition};
use crate::mapping::resolver::entity_resolve_cell::EntityResolveCell;
use crate::mapping::resolver::error::{ResolveError, UnresolvedError};
use crate::mapping::resolver::{
    ConstructableCell, FieldPath, FieldResolveCell, FieldResolveStatus,
};
use crate::mapping::{Column, DatabaseType, FieldAttribute};
use proc_macro2::{Ident, TokenStream};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use syn::export::ToTokens;
use syn::punctuated::Iter;
use syn::{GenericArgument, PathArguments, PathSegment, Type};

fn unwrap_generic_arguments(segment: &PathSegment) -> Option<Iter<GenericArgument>> {
    match &segment.arguments {
        PathArguments::AngleBracketed(arguments) => Some(arguments.args.iter()),
        _ => None,
    }
}

enum CollectionType {
    /// Array with nested type tokens
    Array(TokenStream),
    /// Map with key type and value type tokens
    Map(TokenStream, TokenStream),
}

impl CollectionType {
    pub fn from_last_segment(segment: &PathSegment) -> Option<Self> {
        let ident_string = segment.ident.to_string();

        match ident_string.as_str() {
            "Vec" => match unwrap_generic_arguments(segment) {
                Some(args) => args
                    .filter_map(|arg| match arg {
                        GenericArgument::Type(ty) => Some(ty.to_token_stream()),
                        _ => None,
                    })
                    .next()
                    .map(CollectionType::Array),
                None => None,
            },
            "HashMap" => match unwrap_generic_arguments(segment) {
                Some(args) => {
                    let mut iter = args.filter_map(|arg| match arg {
                        GenericArgument::Type(ty) => Some(ty.to_token_stream()),
                        _ => None,
                    });

                    let first_argument = iter.next()?;
                    let second_argument = iter.next()?;

                    Some(CollectionType::Map(first_argument, second_argument))
                }
                None => None,
            },
            _ => None,
        }
    }

    pub fn to_database_value_tokens(&self, value_ident: &TokenStream) -> TokenStream {
        quote::quote! {
            yukino::mapping::DatabaseValue::Json(
                serde_json::to_value(&#value_ident).map_err(
                    |e| {
                        let message = e.to_string();
                        yukino::ParseError::new(message.as_str())
                    }
                )?
            )
        }
    }

    pub fn to_value_tokens(&self, value: &TokenStream, field_name: String) -> TokenStream {
        let error_message = format!("Unexpected DatabaseValue on field {}", field_name);
        quote::quote! {
            match #value {
                Some(yukino::mapping::DatabaseValue::Json(json)) => {
                    serde_json::from_value(json.clone()).map_err(
                        |e| {
                            let message = e.to_string();
                            yukino::ParseError::new(message.as_str())
                        }
                    )
                },
                _ => Err(yukino::ParseError::new(#error_message))
            }?
        }
    }
}

pub struct CollectionResolveCell {
    status: FieldResolveStatus,
    ty: Option<CollectionType>,
    entity_name: Option<String>,
    field_ident: Option<Ident>,
    column: Option<Column>,
}

impl ConstructableCell for CollectionResolveCell {
    fn get_seed() -> Self
    where
        Self: Sized,
    {
        CollectionResolveCell {
            status: FieldResolveStatus::Seed,
            ty: None,
            entity_name: None,
            field_ident: None,
            column: None,
        }
    }
}

impl FieldResolveCell for CollectionResolveCell {
    fn weight(&self) -> usize {
        1
    }

    fn get_status(&self) -> FieldResolveStatus {
        self.status.clone()
    }

    fn resolve_fields(
        &mut self,
        _fields: HashMap<FieldPath, &dyn FieldResolveCell, RandomState>,
    ) -> Result<FieldResolveStatus, ResolveError> {
        unreachable!()
    }

    fn resolve_entity(
        &mut self,
        _entity: &EntityResolveCell,
    ) -> Result<FieldResolveStatus, ResolveError> {
        unreachable!()
    }

    fn assemble(
        &mut self,
        _entity: &EntityResolveCell,
    ) -> Result<FieldResolveStatus, ResolveError> {
        unreachable!()
    }

    fn field_name(&self) -> Result<String, UnresolvedError> {
        self.field_ident
            .as_ref()
            .map(|name| name.to_string())
            .ok_or_else(|| UnresolvedError::new("Collection Resolve cell"))
    }

    fn column_names(&self) -> Result<Vec<String>, UnresolvedError> {
        self.field_name().map(|field_name| {
            vec![self
                .column
                .as_ref()
                .map(|column| column.name.clone().unwrap_or_else(|| field_name.clone()))
                .unwrap_or(field_name)]
        })
    }

    fn entity_name(&self) -> Result<String, UnresolvedError> {
        self.entity_name
            .as_ref()
            .cloned()
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))
    }

    fn is_primary_key(&self) -> Result<bool, UnresolvedError> {
        if self.status.is_finished() {
            Ok(false)
        } else {
            Err(UnresolvedError::new("Integer Resolve cell"))
        }
    }

    fn get_foreigner_keys(&self) -> Result<Vec<ForeignKeyDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Finished => Ok(vec![]),
            _ => Err(UnresolvedError::new("Collection Resolve cell")),
        }
    }

    fn get_column_definitions(&self) -> Result<Vec<ColumnDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Finished => Ok(vec![ColumnDefinition {
                name: self.column_names()?[0].clone(),
                column_type: DatabaseType::Json,
                unique: false,
                auto_increase: false,
                is_primary_key: false,
            }]),
            _ => Err(UnresolvedError::new("Integer Resolve cell")),
        }
    }

    fn get_joined_table_definitions(&self) -> Result<Vec<TableDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Finished => Ok(vec![]),
            _ => Err(UnresolvedError::new("Collection Resolve cell")),
        }
    }

    fn convert_to_database_value_token_stream(
        &self,
        value_ident: &Ident,
    ) -> Result<TokenStream, UnresolvedError> {
        let field = self
            .field_ident
            .as_ref()
            .ok_or_else(|| UnresolvedError::new("Collection Resolve cell"))?;
        let field_ident = quote::quote! {
            self.#field
        };

        let column_name = self.column_names()?[0].clone();
        self.ty
            .as_ref()
            .map(|ty| {
                let value = ty.to_database_value_tokens(&field_ident);
                quote::quote! {
                    #value_ident.insert(#column_name.to_string(), #value)
                }
            })
            .ok_or_else(|| UnresolvedError::new("Collection Resolve cell"))
    }

    fn convert_to_value_token_stream(
        &self,
        value_ident: &Ident,
    ) -> Result<TokenStream, UnresolvedError> {
        self.ty
            .as_ref()
            .map(|ty| {
                let column_names = self.column_names().unwrap();
                let column_name = column_names.first().unwrap();
                let value = quote::quote! {
                    {
                        let column_name = #column_name.to_string();
                        #value_ident.get(&column_name)
                    }
                };
                ty.to_value_tokens(
                    &value,
                    format!(
                        "{}::{}",
                        self.entity_name().unwrap(),
                        self.field_name().unwrap()
                    ),
                )
            })
            .ok_or_else(|| UnresolvedError::new("Collection Resolve cell"))
    }

    fn breed(
        &self,
        entity_name: String,
        ident: &Ident,
        attributes: &[FieldAttribute],
        field_type: &Type,
    ) -> Result<Box<dyn FieldResolveCell>, ResolveError> {
        let ty = match field_type {
            Type::Path(type_path) => match type_path.path.segments.iter().rev().next() {
                Some(last_segment) => CollectionType::from_last_segment(&last_segment),
                None => None,
            },
            _ => None,
        }
        .ok_or_else(|| {
            let message = format!(
                "{} is not supported by collection resolve cell",
                field_type.to_token_stream().to_string()
            );
            ResolveError::new(&entity_name, &message)
        })?;

        let column = attributes
            .iter()
            .filter_map(|attr| match attr {
                FieldAttribute::Column(column) => Some(column.clone()),
                _ => None,
            })
            .next();

        Ok(Box::new(CollectionResolveCell {
            status: FieldResolveStatus::Finished,
            ty: Some(ty),
            entity_name: Some(entity_name),
            field_ident: Some(ident.clone()),
            column,
        }))
    }

    fn match_field(&self, _attributes: &[FieldAttribute], field_type: &Type) -> bool {
        match field_type {
            Type::Path(type_path) => match type_path.path.segments.iter().rev().next() {
                Some(last_segment) => CollectionType::from_last_segment(&last_segment).is_some(),
                None => false,
            },
            _ => false,
        }
    }
}
