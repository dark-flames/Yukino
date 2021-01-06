use annotation_rs::{Annotation, AnnotationEnumValue};
use std::collections::HashMap;
/// In Yukino model, every field will be mapping into several column in database. When we work with
/// a Field, we don't need to be concerned about how it is mapped to a Column;the framework will map
/// the operations on the Field to the Column in the appropriate way.

/// Annotation of Table
/// Declare a struct to be a table of database
#[derive(Annotation, Clone)]
pub struct Table {
    /// Table name. If empty, it will be generated based on the name of the struct(
    /// `CamelCase` style struct name will be convert into `snake_case`)
    pub name: Option<String>,
    /// Index annotations of table, mapped by index name
    pub indexes: Option<HashMap<String, Index>>,
}

/// Annotation of Index
/// Define a index in table
#[derive(Annotation, Clone)]
pub struct Index {
    /// Name of fields contained in the index
    pub fields: Vec<String>,
    /// Index method. default to be `b_tree`
    #[field(enum_value = true, default = "b_tree")]
    pub method: IndexMethod,
    /// Is unique index. default to be false
    #[field(default = false)]
    pub unique: bool,
}

/// Index Method enum
/// Variants may change depending on the platform
/// * "hash" is not support on `sqlite`
/// * "gin", "sp_gin", "gist", "brin" is only available on `postgre-sql`
#[derive(AnnotationEnumValue, Clone, Debug)]
pub enum IndexMethod {
    BTree,
    #[cfg(any(feature = "mysql", feature = "postgre-sql"))]
    Hash,
    #[cfg(any(feature = "postgre-sql"))]
    Gin,
    #[cfg(any(feature = "postgre-sql"))]
    #[variant_value("sp_gin")]
    SPGin,
    #[cfg(any(feature = "postgre-sql"))]
    Gist,
    #[cfg(any(feature = "postgre-sql"))]
    Brin,
}
