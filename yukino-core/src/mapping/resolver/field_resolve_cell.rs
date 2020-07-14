use super::entity_resolve_cell::EntityResolveCell;
use super::error::{ResolveError, UnresolvedError};
use crate::mapping::definition::{ColumnDefinition, ForeignKeyDefinition, TableDefinition};
use crate::mapping::{DatabaseValue, FieldAttribute};
use crate::ParseError;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use syn::export::fmt::{Display, Formatter, Result as FMTResult};
use syn::Type;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct FieldPath(pub String, pub String);

impl Display for FieldPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        write!(f, "{}::{}", self.0, self.1)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum FieldResolveStatus {
    /// Finished
    Finished,
    /// Waiting for Assembling to entity resolve cell
    WaitAssemble,
    /// Waiting for entity(entity_name)
    WaitEntity(String),
    /// Wait for fields(entity_name, Vec<field_name>)
    WaitFields(Vec<FieldPath>),
    /// Seed
    Seed,
}

impl FieldResolveStatus {
    pub fn get_fields(&self) -> Option<&Vec<FieldPath>> {
        match self {
            FieldResolveStatus::WaitFields(fields) => Some(fields),
            _ => None,
        }
    }

    pub fn get_entity(&self) -> Option<&String> {
        match self {
            FieldResolveStatus::WaitEntity(entity) => Some(entity),
            _ => None,
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            FieldResolveStatus::Finished => true,
            _ => false,
        }
    }
}

pub trait ConstructableCell {
    fn get_seed() -> Self
    where
        Self: Sized;
}

pub trait FieldResolveCell: ConstructableCell {
    fn weight(&self) -> usize;

    fn get_status(&self) -> FieldResolveStatus;

    /// Will be called at WaitFields status
    fn resolve_fields(
        &mut self,
        fields: HashMap<FieldPath, &dyn FieldResolveCell>,
    ) -> Result<FieldResolveStatus, ResolveError>;

    /// Will be called at WaitEntity status
    fn resolve_entity(
        &mut self,
        entity: &EntityResolveCell,
    ) -> Result<FieldResolveStatus, ResolveError>;

    /// Will be called at WaitAssemble status
    fn assemble(&mut self, entity: &EntityResolveCell) -> Result<FieldResolveStatus, ResolveError>;

    fn field_name(&self) -> Result<String, UnresolvedError>;

    fn column_names(&self) -> Result<Vec<String>, UnresolvedError>;

    fn entity_name(&self) -> Result<String, UnresolvedError>;

    fn field_path(&self) -> Result<FieldPath, UnresolvedError> {
        match self.entity_name() {
            Ok(entity_name) => match self.field_name() {
                Ok(field_name) => Ok(FieldPath(entity_name, field_name)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn is_primary_key(&self) -> Result<bool, UnresolvedError>;

    fn get_foreigner_keys(&self) -> Result<Vec<ForeignKeyDefinition>, UnresolvedError>;

    fn get_column_definitions(&self) -> Result<Vec<ColumnDefinition>, UnresolvedError>;

    fn get_joined_table_definitions(&self) -> Result<Vec<TableDefinition>, UnresolvedError>;

    fn get_data_converter_token_stream(&self) -> Result<TokenStream, UnresolvedError>;

    fn get_data_converter_getter_ident(&self) -> Result<Ident, UnresolvedError> {
        Ok(quote::format_ident!("get_{}_converter", self.field_name()?))
    }

    /// breed from seed
    fn breed(
        &self,
        entity_name: String,
        ident: &Ident,
        attributes: &[FieldAttribute],
        field_type: &Type,
    ) -> Result<Box<dyn FieldResolveCell>, ResolveError>;

    /// Return true if cell matched the field
    fn match_field(&self, attributes: &[FieldAttribute], field_type: &Type) -> bool;
}

pub trait ValueConverter<T>: ToTokens {
    fn to_value(&self, values: &HashMap<String, DatabaseValue>) -> Result<T, ParseError>;

    fn to_database_value(&self, value: T) -> Result<HashMap<String, DatabaseValue>, ParseError> {
        self.to_database_value_by_ref(&value)
    }

    fn to_database_value_by_ref(
        &self,
        value: &T,
    ) -> Result<HashMap<String, DatabaseValue>, ParseError>;
}
