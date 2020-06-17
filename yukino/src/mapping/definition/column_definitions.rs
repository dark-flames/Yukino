use crate::mapping::definition::table_definitions::IndexDefinition;
use super::super::r#type::DatabaseType;
use crate::mapping::attribution::{ReferenceAction, FetchMode};

#[allow(dead_code)]
pub enum Column {
    NormalColumn(ColumnDefinition),
    InternalColumn(IndexDefinition),
    VirtualColumn(VirtualColumnDefinition)
}

#[allow(dead_code)]
pub struct ColumnDefinition {
    pub name: String,
    pub field_name: String,
    pub column_type: DatabaseType
}

#[allow(dead_code)]
pub struct InternalColumnDefinition {
    pub name: String,
    pub column_type: DatabaseType,
    pub reference_table: String,
    pub reference_column: String
}

pub struct VirtualColumnDefinition {
    pub field_name: String,
    pub reference_table: String,
    pub reference_column: String,
    pub type_table_name: String,
    pub is_list: bool,
    pub on_delete: ReferenceAction,
    pub on_update: ReferenceAction,
    pub fetch_mode: FetchMode
}

