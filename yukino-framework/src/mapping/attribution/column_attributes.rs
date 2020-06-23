use yui::YuiAttribute;
use std::collections::HashMap;


/// Announce column as primary key.
/// It can be used on field of entity struct.
/// ```
/// // todo: some test
/// ```
/// Yukino also support multi-primary-key, but a table must have one primary key at least.
#[derive(YuiAttribute, Clone)]
pub struct Id;


/// Announce field as a column of table.
#[derive(YuiAttribute, Clone)]
pub struct Column {
    /// Name of column, default is field ident(will be converted to `snake_case`) in Rust.
    pub name: Option<String>,
    /// Is column unique.
    #[attribute_field(default=false)]
    pub unique: Option<bool>,
    /// Auto increase
    #[attribute_field(default=false)]
    pub auto_increase: bool,
    /// Options
    pub options: Option<HashMap<String, String>>
}