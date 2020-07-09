use crate::mapping::{DatabaseType, Index, IndexMethod, ReferenceAction};
use iroha::ToTokens;

/// DataStructure of Column in table
#[derive(Clone, ToTokens, Debug, Eq, PartialEq)]
#[allow(dead_code)]
#[Iroha(mod_path = "yukino::mapping::definition")]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: DatabaseType,
    pub unique: bool,
    pub auto_increase: bool,
    pub is_primary_key: bool,
}

/// DataStructure of Index in table
#[allow(dead_code)]
#[derive(Clone, ToTokens, Debug, Eq, PartialEq)]
#[Iroha(mod_path = "yukino::mapping::definition")]
pub struct IndexDefinition {
    pub name: String,
    pub method: IndexMethod,
    pub columns: Vec<String>,
    pub unique: bool,
}

impl IndexDefinition {
    pub fn from_attr(name: &str, attr: &Index) -> Self {
        IndexDefinition {
            name: name.to_string(),
            method: attr.method.clone(),
            columns: attr.columns.clone(),
            unique: attr.unique,
        }
    }
}

/// DataStructure of ForeignKey in table
#[allow(dead_code)]
#[derive(Clone, ToTokens, Debug, Eq, PartialEq)]
#[Iroha(mod_path = "yukino::mapping::definition")]
pub struct ForeignKeyDefinition {
    pub name: String,
    pub columns: Vec<String>,
    pub reference_table: String,
    pub reference_columns: Vec<String>,
    pub on_update: ReferenceAction,
    pub on_delete: ReferenceAction,
}

/// DataStructure of table
#[allow(dead_code)]
#[derive(Clone, ToTokens, Debug, Eq, PartialEq)]
#[Iroha(mod_path = "yukino::mapping::definition")]
pub struct TableDefinition {
    pub name: String,
    pub indexes: Vec<IndexDefinition>,
    pub columns: Vec<ColumnDefinition>,
    pub foreign_keys: Vec<ForeignKeyDefinition>,
}
