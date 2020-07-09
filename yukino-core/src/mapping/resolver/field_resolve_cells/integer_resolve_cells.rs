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

pub enum IntegerType {
    Integer(usize),
    UnsignedInteger(usize),
}

impl IntegerType {
    pub fn from_ident(ident: &Ident) -> Option<Self> {
        let ident_string = ident.to_string();

        match ident_string.as_str() {
            "i16" => Some(IntegerType::Integer(16)),
            "i32" => Some(IntegerType::Integer(32)),
            "i64" => Some(IntegerType::Integer(64)),
            "u16" => Some(IntegerType::UnsignedInteger(16)),
            "u32" => Some(IntegerType::UnsignedInteger(32)),
            "u64" => Some(IntegerType::UnsignedInteger(64)),
            _ => None,
        }
    }

    pub fn get_database_type(&self) -> DatabaseType {
        match self {
            IntegerType::Integer(length) => match length {
                8 => DatabaseType::SmallInteger,
                16 | 32 => DatabaseType::Integer,
                64 => DatabaseType::BigInteger,
                _ => unreachable!(),
            },
            IntegerType::UnsignedInteger(length) => match length {
                8 => DatabaseType::UnsignedSmallInteger,
                16 | 32 => DatabaseType::UnsignedInteger,
                64 => DatabaseType::UnsignedBigInteger,
                _ => unreachable!(),
            },
        }
    }

    fn database_value_variant(&self) -> TokenStream {
        match self {
            IntegerType::Integer(16) => quote::quote! {
                yukino::mapping::DatabaseValue::SmallInteger
            },
            IntegerType::Integer(32) => quote::quote! {
                yukino::mapping::DatabaseValue::Integer
            },
            IntegerType::Integer(64) => quote::quote! {
                yukino::mapping::DatabaseValue::BigInteger
            },
            IntegerType::UnsignedInteger(16) => quote::quote! {
                yukino::mapping::DatabaseValue::UnsignedSmallInteger
            },
            IntegerType::UnsignedInteger(32) => quote::quote! {
                yukino::mapping::DatabaseValue::UnsignedInteger
            },
            IntegerType::UnsignedInteger(64) => quote::quote! {
                yukino::mapping::DatabaseValue::UnsignedBigInteger
            },
            _ => unreachable!(),
        }
    }

    pub fn to_database_value_tokens(&self, value_ident: &TokenStream) -> TokenStream {
        let variant = self.database_value_variant();
        quote::quote! {
            #variant(#value_ident)
        }
    }

    pub fn to_value_tokens(&self, value: &TokenStream, field_name: String) -> TokenStream {
        let variant = self.database_value_variant();
        let convert = match self {
            IntegerType::Integer(16) => quote::quote! {
                as i16
            },
            IntegerType::UnsignedInteger(16) => quote::quote! {
                as u16
            },
            _ => quote::quote! {},
        };
        let error_message = format!("Unexpected DatabaseValue on field {}", field_name);
        quote::quote! {
            match #value {
                Some(#variant(integer)) => Ok(*integer #convert),
                _ => Err(yukino::ParseError::new(#error_message))
            }?
        }
    }
}

pub struct IntegerResolveCell {
    status: FieldResolveStatus,
    entity_name: Option<String>,
    field_ident: Option<Ident>,
    is_primary_key: Option<bool>,
    column: Option<Column>,
    ty: Option<IntegerType>,
}

impl ConstructableCell for IntegerResolveCell {
    fn get_seed() -> Self
    where
        Self: Sized,
    {
        IntegerResolveCell {
            status: FieldResolveStatus::Seed,
            entity_name: None,
            field_ident: None,
            is_primary_key: None,
            column: None,
            ty: None,
        }
    }
}

impl FieldResolveCell for IntegerResolveCell {
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
        unimplemented!()
    }

    fn field_name(&self) -> Result<String, UnresolvedError> {
        self.field_ident
            .as_ref()
            .map(|name| name.to_string())
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))
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
        self.is_primary_key
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))
    }

    fn get_foreigner_keys(&self) -> Result<Vec<ForeignKeyDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Seed => Err(UnresolvedError::new("Integer Resolve cell")),
            _ => Ok(vec![]),
        }
    }

    fn get_column_definitions(&self) -> Result<Vec<ColumnDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Seed => Err(UnresolvedError::new("Integer Resolve cell")),
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
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))
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
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))
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
                Some(first_segment) => IntegerType::from_ident(&first_segment.ident),
                None => None,
            },
            _ => None,
        }
        .ok_or_else(|| {
            let message = format!(
                "{} is not supported by integer resolve cell",
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

        Ok(Box::new(IntegerResolveCell {
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
                Some(first_segment) => IntegerType::from_ident(&first_segment.ident).is_some(),
                None => false,
            },
            _ => false,
        }
    }
}
