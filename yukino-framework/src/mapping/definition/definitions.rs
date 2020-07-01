use crate::mapping::r#type::DatabaseType;
use crate::mapping::attribution::{IndexMethod, ReferenceAction, Index};
use iroha::ToTokens;

#[allow(dead_code)]
#[derive(ToTokens)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: DatabaseType,
    pub unique: bool,
    pub auto_increase: bool,
    pub is_primary_key: bool
}

#[allow(dead_code)]
#[derive(Clone, ToTokens)]
pub struct IndexDefinition {
    pub name: String,
    pub method: IndexMethod,
    pub columns: Vec<String>,
    pub unique: bool
}

impl IndexDefinition {
    pub fn from_attr(name: &str, attr: &Index) -> Self {
        IndexDefinition {
            name: name.to_string(),
            method: attr.method.clone(),
            columns: attr.columns.clone(),
            unique: attr.unique
        }
    }
}

#[allow(dead_code)]
#[derive(ToTokens)]
pub struct ForeignKeyDefinition {
    pub name: String,
    pub columns: Vec<String>,
    pub reference_table: String,
    pub reference_columns: Vec<String>,
    pub on_update: ReferenceAction,
    pub on_delete: ReferenceAction
}

#[allow(dead_code)]
#[derive(ToTokens)]
pub struct TableDefinition {
    pub name: String,
    pub indexes: Vec<IndexDefinition>,
    pub columns: Vec<ColumnDefinition>,
    pub foreign_keys: Vec<ForeignKeyDefinition>
}