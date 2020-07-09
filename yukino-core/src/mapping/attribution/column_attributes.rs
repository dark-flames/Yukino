use std::collections::HashMap;
use yui::YuiAttribute;

/// Announce column as primary key.
/// It can be used on field of entity struct.
/// Yukino also support multi-primary-key, but a table must have one primary key at least.
#[derive(YuiAttribute, Clone)]
pub struct Id;

/// Options of column
/// If a field doesn't have Column attribute, the column will be generate automatically
#[derive(YuiAttribute, Clone)]
pub struct Column {
    /// Name of column, default is field ident(will be converted to `snake_case`) in Rust.
    pub name: Option<String>,
    /// Is column unique.
    #[attribute_field(default = false)]
    pub unique: bool,
    /// Auto increase
    #[attribute_field(default = false)]
    pub auto_increase: bool,
    /// Options required by field resolve cells
    pub options: Option<HashMap<String, String>>,
}

/// Ignore field
#[derive(YuiAttribute, Clone)]
pub struct Ignore;
