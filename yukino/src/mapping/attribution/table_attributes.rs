use yui::YuiAttribute;
use std::collections::HashMap;
use super::enums::*;

#[derive(YuiAttribute, Clone)]
struct Index {
    pub columns: Option<Vec<String>>,
    #[attribute_field(enum_value=true, default="b-tree")]
    pub method: IndexMethod,
    #[attribute_field(default=false)]
    pub unique: bool
}

#[derive(YuiAttribute, Clone)]
struct Table {
    pub name: Option<String>,
    pub indexes: Option<HashMap<String, Index>>
}




