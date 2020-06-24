use proc_macro2::Ident;
use crate::mapping::attribution::{Table};
use crate::mapping::definition::definitions::{ColumnDefinition, IndexDefinition, TableDefinition, ForeignKeyDefinition};
use std::collections::HashMap;
use crate::mapping::resolver::field_resolve_cell::FieldResolveCell;
use crate::mapping::r#type::DatabaseType;
use heck::SnakeCase;
use crate::mapping::resolver::error::{UnresolvedError, ResolveError};

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
pub enum EntityResolveStatus {
    Finished,
    Assembly,
    Unresolved
}

pub struct EntityResolveCell {
    status: EntityResolveStatus,
    ident: Ident,
    field_count: usize,
    name: String,
    indexes: Vec<IndexDefinition>,
    primary_keys: Vec<String>,
    fields: HashMap<String, Box<dyn FieldResolveCell>>
}

#[allow(dead_code)]
impl<'a> EntityResolveCell {
    pub fn get_status(&self) -> EntityResolveStatus {
        self.status.clone()
    }

    pub fn new(ident: &Ident, attr: &Option<Table>, field_count: usize) -> Result<Self, ResolveError> {
        let indexes = if let Some(table_attr) = attr {
            let default = HashMap::new();
            table_attr.indexes.as_ref().unwrap_or(&default).iter().map(
                |(name, index)| IndexDefinition::from_attr(name, index)
            ).collect()
        } else {
            Vec::new()
        };
        let ident_name = ident.to_string().to_snake_case();
        let name = if let Some(table_attr) = attr {
            table_attr.name.clone()
                .unwrap_or(ident_name)
        } else {
            ident_name
        };


        Ok(EntityResolveCell {
            status: EntityResolveStatus::Assembly,
            ident: ident.clone(),
            field_count,
            name,
            indexes,
            primary_keys: Vec::new(),
            fields: HashMap::new()
        })
    }

    pub fn entity_name(&self) -> String {
        self.ident.to_string()
    }

    pub fn get_field(&self, name: &str) -> Option<&dyn FieldResolveCell> {
        self.fields.get(name).map(
            |field| field.as_ref()
        )
    }

    pub fn assemble_column(&mut self, cell: Box<dyn FieldResolveCell>) -> EntityResolveStatus {
        if cell.is_primary_key().unwrap() {
            let mut names = cell.column_names();
            self.primary_keys.append(&mut names)
        }

        self.fields.insert(cell.field_name(), cell);

        self.status = if self.fields.len() == self.field_count {
            EntityResolveStatus::Finished
        } else {
            EntityResolveStatus::Assembly
        };

        self.status.clone()
    }

    pub fn get_primary_keys(&self) -> Result<&Vec<String>, UnresolvedError> {
        match self.get_status() {
            EntityResolveStatus::Finished => Ok(&self.primary_keys),
            _ => Err(UnresolvedError::new(&self.ident))
        }
    }

    pub fn achieve(&mut self) -> Result<Vec<TableDefinition>, UnresolvedError> {
        if self.status != EntityResolveStatus::Finished {
            return Err(UnresolvedError::new(&self.ident.to_string()))
        };

        let mut columns: Vec<ColumnDefinition> = Vec::new();
        let mut tables: Vec<TableDefinition> = Vec::new();
        let mut foreign_keys: Vec<ForeignKeyDefinition> = Vec::new();
        if self.primary_keys.is_empty() {
            let auto_primary_keys = ColumnDefinition {
                name: format!("__{}_id", self.ident.to_string().to_snake_case()),
                column_type: DatabaseType::String,
                unique: true,
                auto_increase: false,
                is_primary_key: true
            };
            self.primary_keys.push(auto_primary_keys.name.clone());
            columns.push(auto_primary_keys);
        }

        for (_, cell) in self.fields.iter() {
            let mut cell_columns = cell.get_column_definitions()?;
            let mut cell_tables = cell.get_joined_table_definitions()?;
            let mut cell_foreign_keys = cell.get_foreigner_keys()?;
            columns.append(&mut cell_columns);
            tables.append(&mut cell_tables);
            foreign_keys.append(&mut cell_foreign_keys);
        }

        tables.push(TableDefinition {
            name: self.name.clone(),
            indexes: self.indexes.clone(),
            columns,
            foreign_keys
        });

        self.status = EntityResolveStatus::Finished;

        Ok(tables)
    }
}