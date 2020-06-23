use crate::mapping::attribution::{FieldAttribute};
use syn::Type;
use proc_macro2::{Ident, TokenStream};
use std::collections::HashMap;
use crate::mapping::resolver::error::{UnresolvedError, ResolveError};
use crate::mapping::definition::definitions::{ColumnDefinition, TableDefinition, ForeignKeyDefinition};
use crate::mapping::resolver::entity_resolve_cell::EntityResolveCell;
use syn::export::fmt::{Display, Formatter, Result as FMTResult};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct FieldPath (pub String, pub String);

impl Display for FieldPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        write!(f, "{}::{}", self.0, self.1)
    }
}

#[allow(dead_code)]
pub enum FieldResolveStatus {
    /// finished
    Finished,

    WaitAssembly,
    /// Wait for entity(entity_name)
    WaitEntity(String),
    /// Wait for fields(entity_name, Vec<field_name>)
    WaitFields(Vec<FieldPath>),

    Unresolved
}

impl FieldResolveStatus {
    pub fn get_fields(&self) -> Option<&Vec<FieldPath>> {
        match self {
            FieldResolveStatus::WaitFields(fields) => Some(fields),
            _ => None
        }
    }

    pub fn get_entity(&self) -> Option<&String> {
        match self {
            FieldResolveStatus::WaitEntity(entity) => Some(entity),
            _ => None
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            FieldResolveStatus::Finished => true,
            _ => false
        }
    }
}

pub trait FieldResolveCell: {
    fn weight(&self) -> usize;

    fn logical_type(&self) -> String;

    fn get_status(&self) -> FieldResolveStatus;

    fn resolve_fields(&mut self, fields: HashMap<FieldPath, &Box<dyn FieldResolveCell>>) -> Result<FieldResolveStatus, ResolveError>;

    fn resolve_entity(&mut self, entity: &EntityResolveCell) -> Result<FieldResolveStatus, ResolveError>;

    fn assembly(&mut self, entity: &EntityResolveCell) -> Result<FieldResolveStatus, ResolveError>;

    fn field_name(&self) -> String;

    fn column_names(&self) -> Vec<String>;

    fn entity_name(&self) -> String;

    fn field_path(&self) -> FieldPath {
        FieldPath(self.entity_name() , self.field_name())
    }

    fn is_primary_key(&self) -> Result<bool, UnresolvedError>;

    fn get_foreigner_keys(&self) -> Result<Vec<ForeignKeyDefinition>, UnresolvedError>;

    fn get_column_definitions(&self) -> Result<Vec<ColumnDefinition>, UnresolvedError>;

    fn get_joined_table_definitions(&self) -> Result<Vec<TableDefinition>, UnresolvedError>;

    fn convert_to_database_value_token_stream(&self) -> Result<TokenStream, UnresolvedError>;

    fn convert_to_value_token_stream(&self) -> Result<TokenStream, UnresolvedError>;

    fn breed(&self, entity_name: &Ident, ident: &Ident, attributes: &Vec<FieldAttribute>, field_type: &Type) -> Result<Box<dyn FieldResolveCell>, ResolveError>;

    fn match_field(&self, attributes: &Vec<FieldAttribute>, field_type: &Type) -> bool;
}