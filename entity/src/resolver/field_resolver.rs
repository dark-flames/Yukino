use crate::annotations::FieldAnnotation;
use crate::definitions::{ColumnDefinition, ForeignKeyDefinition, TableDefinition};
use crate::resolver::error::ResolveError;
use crate::resolver::{EntityPath, EntityResolver, FieldPath};
use proc_macro2::{Ident, TokenStream};
use syn::Type;

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
    fn breed(
        &self,
        entity_path: EntityPath,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
    ) -> Result<FieldResolverBox, ResolveError>;

    fn match_field(&self, annotations: &[FieldAnnotation], field_type: &Type) -> bool;
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

pub struct AchievedFieldResolver {
    pub field_path: FieldPath,
    pub columns: Vec<ColumnDefinition>,
    pub joined_table: Vec<TableDefinition>,
    pub foreign_keys: Vec<ForeignKeyDefinition>,
    pub data_converter_token_stream: TokenStream,
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

    pub fn data_converter_getter_ident(&self) -> Ident {
        quote::format_ident!("get_{}_converter", &self.field_path.1)
    }
}
