use crate::mapping::definition::{ColumnDefinition, ForeignKeyDefinition, TableDefinition};
use crate::mapping::resolver::entity_resolve_cell::EntityResolveCell;
use crate::mapping::resolver::error::{ResolveError, UnresolvedError};
use crate::mapping::resolver::{
    ConstructableCell, FieldPath, FieldResolveCell, FieldResolveStatus,
};
use crate::mapping::{Column, DatabaseType, FieldAttribute};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use syn::Type;

struct FloatType(usize);

impl FloatType {
    pub fn from_ident(ident: &Ident) -> Option<Self> {
        let ident_string = ident.to_string();

        match ident_string.as_str() {
            "f32" => Some(FloatType(32)),
            "f64" => Some(FloatType(64)),
            _ => None,
        }
    }

    pub fn get_database_type(&self) -> DatabaseType {
        match self {
            FloatType(length) => match length {
                32 => DatabaseType::Float,
                64 => DatabaseType::Double,
                _ => unreachable!(),
            },
        }
    }

    fn database_value_variant(&self) -> TokenStream {
        match self {
            FloatType(32) => quote::quote! {
                yukino::mapping::DatabaseValue::Float
            },
            FloatType(64) => quote::quote! {
                yukino::mapping::DatabaseValue::Double
            },
            _ => unreachable!(),
        }
    }

    pub fn to_database_value_tokens(&self, value_ident: &TokenStream) -> TokenStream {
        let param = match self {
            FloatType(32) => quote::quote! {
                (#value_ident)
            },
            FloatType(64) => quote::quote! {
                (#value_ident)
            },
            _ => quote::quote! {
                (#value_ident)
            },
        };
        let variant = self.database_value_variant();
        quote::quote! {
            #variant#param
        }
    }

    pub fn to_value_tokens(&self, value: &TokenStream, field_name: String) -> TokenStream {
        let variant = self.database_value_variant();
        let error_message = format!("Unexpected DatabaseValue on field {}", field_name);
        quote::quote! {
            match #value {
                Some(#variant(float)) => Ok(*float),
                _ => Err(yukino::ParseError::new(#error_message))
            }?
        }
    }
}

pub struct FloatResolveCell {
    status: FieldResolveStatus,
    entity_name: Option<String>,
    field_ident: Option<Ident>,
    is_primary_key: Option<bool>,
    column: Option<Column>,
    ty: Option<FloatType>,
}

impl ConstructableCell for FloatResolveCell {
    fn get_seed() -> Self
    where
        Self: Sized,
    {
        FloatResolveCell {
            status: FieldResolveStatus::Seed,
            entity_name: None,
            field_ident: None,
            is_primary_key: None,
            column: None,
            ty: None,
        }
    }
}

impl FieldResolveCell for FloatResolveCell {
    fn weight(&self) -> usize {
        1
    }

    fn get_status(&self) -> FieldResolveStatus {
        self.status.clone()
    }

    fn resolve_fields(
        &mut self,
        _fields: HashMap<FieldPath, &dyn FieldResolveCell>,
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
            .ok_or_else(|| UnresolvedError::new("Float Resolve cell"))
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
            .ok_or_else(|| UnresolvedError::new("Float Resolve cell"))
    }

    fn is_primary_key(&self) -> Result<bool, UnresolvedError> {
        self.is_primary_key
            .ok_or_else(|| UnresolvedError::new("Float Resolve cell"))
    }

    fn get_foreigner_keys(&self) -> Result<Vec<ForeignKeyDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Seed => Err(UnresolvedError::new("Float Resolve cell")),
            _ => Ok(vec![]),
        }
    }

    fn get_column_definitions(&self) -> Result<Vec<ColumnDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Seed => Err(UnresolvedError::new("Float Resolve cell")),
            _ => Ok(vec![ColumnDefinition {
                name: self.column_names()?[0].clone(),
                column_type: self.ty.as_ref().unwrap().get_database_type(),
                unique: self
                    .column
                    .as_ref()
                    .map(|column| column.unique)
                    .unwrap_or(false),
                auto_increase: self
                    .column
                    .as_ref()
                    .map(|column| column.auto_increase)
                    .unwrap_or(false),
                is_primary_key: self.is_primary_key.unwrap(),
            }]),
        }
    }

    fn get_joined_table_definitions(&self) -> Result<Vec<TableDefinition>, UnresolvedError> {
        Ok(vec![])
    }

    fn convert_to_database_value_token_stream(
        &self,
        value_ident: &Ident,
    ) -> Result<TokenStream, UnresolvedError> {
        let field = self
            .field_ident
            .as_ref()
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))?;
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
            .ok_or_else(|| UnresolvedError::new("Float Resolve cell"))
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
                ty.to_value_tokens(&value, self.entity_name().unwrap())
            })
            .ok_or_else(|| UnresolvedError::new("Float Resolve cell"))
    }

    fn breed(
        &self,
        entity_name: String,
        ident: &Ident,
        attributes: &[FieldAttribute],
        field_type: &Type,
    ) -> Result<Box<dyn FieldResolveCell>, ResolveError> {
        let ty = match field_type {
            Type::Path(type_path) => match type_path.path.segments.first() {
                Some(first_segment) => FloatType::from_ident(&first_segment.ident),
                None => None,
            },
            _ => None,
        }
        .ok_or_else(|| {
            let message = format!(
                "{} is not supported by Float resolve cell",
                field_type.to_token_stream().to_string()
            );
            ResolveError::new(&entity_name, &message)
        })?;

        let is_primary_key = attributes.iter().any(|attr| {
            if let FieldAttribute::Id(_) = attr {
                true
            } else {
                false
            }
        });

        let column = attributes
            .iter()
            .filter_map(|attr| match attr {
                FieldAttribute::Column(column) => Some(column.clone()),
                _ => None,
            })
            .next();

        Ok(Box::new(FloatResolveCell {
            status: FieldResolveStatus::Finished,
            entity_name: Some(entity_name),
            field_ident: Some(ident.clone()),
            is_primary_key: Some(is_primary_key),
            column,
            ty: Some(ty),
        }))
    }

    fn match_field(&self, _attributes: &[FieldAttribute], field_type: &Type) -> bool {
        match field_type {
            Type::Path(type_path) => match type_path.path.segments.first() {
                Some(first_segment) => FloatType::from_ident(&first_segment.ident).is_some(),
                None => false,
            },
            _ => false,
        }
    }
}
