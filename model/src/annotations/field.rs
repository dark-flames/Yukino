#[doc(hidden)]
use annotation_rs::Annotation;
use std::collections::HashMap;

/// Announce a field as primary key.
/// It can be used on field of entity.
/// Yukino also supports multiple primary keys, but if a entity does not specify a primary key,
/// Yukino will automatically generate a uuid column as the primary key.
#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct ID;

/// Annotation of field.
/// It can be used on field of entity.
/// If a field doesn't have a Field annotation, it will be generate automatically.
#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct Field {
    /// Column name(maybe prefix). If empty, it will be generated based on the name of the field(
    /// `CamelCase` style struct name will be convert into `snake_case`).
    pub name: Option<String>,
    /// Is unique field. default to be false.
    #[field(default = false)]
    pub unique: bool,
    /// Auto increase field. default to be false.
    #[field(default = false)]
    pub auto_increase: bool,
    /// Optional config.
    pub options: Option<HashMap<String, String>>,
}

/// Ignore field of struct.
#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct Ignore;
