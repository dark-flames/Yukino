use crate::mapping::attribution::Table;
use syn::Visibility;
use proc_macro2::Ident;
use std::collections::HashMap;
use crate::mapping::structure::field_structure::FieldStructure;
use crate::mapping::definition::table_definitions::NormalTableDefinition;

#[allow(dead_code)]
pub struct EntityStructure<'a> {
    pub table_attr: Table,
    pub visibility: &'a Visibility,
    pub ident: Ident,
    pub fields: HashMap<String, Box<FieldStructure<'a>>>,
    pub resolved: bool,
    pub waiting_for: Option<String>,
    pub table_definition: Option<NormalTableDefinition>
}

impl EntityStructure<'_> {
}