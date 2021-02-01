use crate::annotations::{Field, FieldAnnotation};
use crate::definitions::{
    ColumnDefinition, ForeignKeyDefinition, IndexDefinition, TableDefinition,
};
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{EntityName, EntityResolver, FieldPath, TypePathResolver};
use crate::types::ValuePack;
use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::fmt;
use std::fmt::{Display, Formatter};
use syn::Type;

#[derive(Clone, Debug)]
pub enum FieldResolverStatus {
    WaitingForFields(Vec<FieldPath>),
    WaitingForEntity(EntityName),
    WaitingAssemble,
}

impl Display for FieldResolverStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FieldResolverStatus {
    pub fn is_waiting_for_entity(&self, entity_name: &str) -> bool {
        matches!(self, FieldResolverStatus::WaitingForEntity(path) if path == entity_name)
    }
}

pub type FieldResolverBox = Box<dyn FieldResolver>;
pub type FieldResolverSeedBox = Box<dyn FieldResolverSeed>;

pub trait FieldResolverSeed {
    fn new() -> Self
    where
        Self: Sized;

    fn boxed(&self) -> FieldResolverSeedBox;

    fn try_breed(
        &self,
        entity_name: EntityName,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
        type_path_resolver: &TypePathResolver,
    ) -> Option<Result<FieldResolverBox, ResolveError>>;

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

    fn converter_getter_ident(&self) -> Ident {
        quote::format_ident!("get_{}_converter", &self.field_path().1.to_snake_case())
    }

    fn getter_ident(&self) -> Ident {
        quote::format_ident!("get_{}", &self.field_path().1.to_snake_case())
    }

    fn setter_ident(&self) -> Ident {
        quote::format_ident!("set_{}", &self.field_path().1.to_snake_case())
    }
}

pub trait ValueConverter<T>: ToTokens {
    fn to_field_value(&self, values: &ValuePack) -> Result<T, DataConvertError>;

    fn to_database_values(&self, value: T) -> Result<ValuePack, DataConvertError> {
        self.to_database_values_by_ref(&value)
    }

    fn to_database_values_by_ref(&self, value: &T) -> Result<ValuePack, DataConvertError>;

    fn primary_column_values_by_ref(&self, value: &T) -> Result<ValuePack, DataConvertError>;
}

pub struct AchievedFieldResolver {
    pub field_path: FieldPath,
    pub indexes: Vec<IndexDefinition>,
    pub columns: Vec<ColumnDefinition>,
    pub joined_table: Vec<TableDefinition>,
    pub foreign_keys: Vec<ForeignKeyDefinition>,
    pub data_converter_token_stream: TokenStream,
    pub converter_getter_ident: Ident,
    pub field_getter_ident: Ident,
    pub field_getter_token_stream: TokenStream,
    pub field_setter_ident: Ident,
    pub field_setter_token_stream: TokenStream,
    pub field_type: Type,
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

    pub fn unique(&self) -> bool {
        self.columns
            .iter()
            .map(|definitions| definitions.unique)
            .any(|is_unique| is_unique)
    }

    pub fn primary_key(&self) -> bool {
        self.columns
            .iter()
            .map(|definitions| definitions.primary_key)
            .any(|is_unique| is_unique)
    }
}
