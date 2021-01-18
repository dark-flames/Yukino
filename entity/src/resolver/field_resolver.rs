use crate::annotations::{Field, FieldAnnotation};
use crate::definitions::{ColumnDefinition, ForeignKeyDefinition, TableDefinition};
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{EntityPath, EntityResolver, FieldPath};
use crate::types::DatabaseValue;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use syn::Type;

#[derive(Debug)]
pub enum FieldResolverStatus {
    WaitingForFields(Vec<FieldPath>),
    WaitingForEntity(EntityPath),
    WaitingAssemble,
}

impl FieldResolverStatus {
    pub fn is_waiting_for_entity(&self, entity_path: &str) -> bool {
        matches!(self, FieldResolverStatus::WaitingForEntity(path) if path == entity_path)
    }
}

pub type FieldResolverBox = Box<dyn FieldResolver>;
pub type FieldResolverSeedBox = Box<dyn FieldResolverSeed>;

pub trait ConstructableFieldResolverSeed {
    fn new() -> Self
    where
        Self: Sized;
}

pub trait FieldResolverSeed: ConstructableFieldResolverSeed {
    fn try_breed(
        &self,
        entity_path: EntityPath,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
    ) -> Result<FieldResolverBox, ResolveError>;

    fn default_annotations(annotations: &[FieldAnnotation]) -> Field
    where
        Self: Sized,
    {
        let default_annotation = Field {
            name: None,
            unique: false,
            auto_increase: false,
            options: None,
        };

        annotations
            .iter()
            .filter_map(|attr| match attr {
                FieldAnnotation::Field(field_annotation) => Some(field_annotation),
                _ => None,
            })
            .next()
            .cloned()
            .unwrap_or(default_annotation)
    }

    fn is_primary_key(annotations: &[FieldAnnotation]) -> bool
    where
        Self: Sized,
    {
        annotations
            .iter()
            .any(|attr| matches!(attr, FieldAnnotation::ID(_)))
    }
}

pub trait FieldResolver {
    fn status(&self) -> FieldResolverStatus;

    fn field_path(&self) -> FieldPath;

    fn entity_path(&self) -> EntityPath;

    fn resolve_by_waiting_entity(
        &mut self,
        resolver: &EntityResolver,
    ) -> Result<FieldResolverStatus, ResolveError>;

    fn resolve_by_waiting_fields(
        &mut self,
        resolvers: Vec<&AchievedFieldResolver>,
    ) -> Result<FieldResolverStatus, ResolveError>;

    fn assemble(
        &mut self,
        entity_resolver: &EntityResolver,
    ) -> Result<AchievedFieldResolver, ResolveError>;
}

pub trait ValueConverter<T>: ToTokens {
    fn to_value(&self, values: &HashMap<String, DatabaseValue>) -> Result<T, DataConvertError>;

    fn to_database_value(
        &self,
        value: T,
    ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
        self.to_database_value_by_ref(&value)
    }

    fn to_database_value_by_ref(
        &self,
        value: &T,
    ) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;
}

pub struct AchievedFieldResolver {
    pub field_path: FieldPath,
    pub columns: Vec<ColumnDefinition>,
    pub joined_table: Vec<TableDefinition>,
    pub foreign_keys: Vec<ForeignKeyDefinition>,
    pub data_converter_token_stream: TokenStream,
    pub converter_getter_ident: Ident,
}

impl AchievedFieldResolver {
    pub fn primary_key_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .filter_map(|column| {
                if column.primary_key {
                    Some(column.name.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .map(|column| column.name.clone())
            .collect()
    }
}
