use crate::mapping::r#type::DatabaseType;
use crate::mapping::attribution::{IndexMethod, ReferenceAction};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: DatabaseType,
    pub logical_type: String,
    pub unique: bool,
    pub auto_increase: bool,
    pub is_primary_key: bool
}

#[allow(dead_code)]
pub struct IndexDefinition {
    pub name: String,
    pub method: IndexMethod,
    pub columns: Vec<String>,
    pub unique: bool
}

#[allow(dead_code)]
pub struct ForeignKey {
    pub name: String,
    pub columns: Vec<String>,
    pub reference_table: String,
    pub reference_columns: Vec<String>,
    pub on_update: ReferenceAction,
    pub on_delete: ReferenceAction
}

#[allow(dead_code)]
pub struct TableDefinition {
    pub name: String,
    pub table_type: String,
    pub indexes: HashMap<String, ColumnDefinition>,
    pub columns: Vec<ColumnDefinition>,
    pub foreign_keys: Vec<ForeignKey>
}