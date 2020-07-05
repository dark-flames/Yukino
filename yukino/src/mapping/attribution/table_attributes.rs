use yui::YuiAttribute;
use std::collections::HashMap;
use super::enums::*;


/// DataStructure of index.
///
/// It can be used in attribute `Table`.
/// ```
/// // todo: some test
/// ```
///
#[derive(YuiAttribute, Clone)]
pub struct Index {
    /// Column names(field name in Rust) of index.
    pub columns: Vec<String>,
    /// Index method, enum `IndexMethod` will be different in different platform. Default use b-tree
    #[attribute_field(enum_value=true, default="b_tree")]
    pub method: IndexMethod,
    /// Is unique index. Default false.
    #[attribute_field(default=false)]
    pub unique: bool
}

/// DataStructure of Table
///
/// Used to alias default table name or add index to table.
#[derive(YuiAttribute, Clone)]
pub struct Table {
    /// Table name in database, default is the struct ident(will be converted to snake_case) in Rust
    pub name: Option<String>,
    /// Index definitions, mapped by index name.
    pub indexes: Option<HashMap<String, Index>>
}




