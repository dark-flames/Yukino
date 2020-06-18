use super::super::attribution::{Column, AssociateColumn, InverseAssociateColumn};
use syn::{Type, Visibility};
use proc_macro2::Ident;
use crate::mapping::definition::column_definitions::{ColumnDefinition, InternalColumnDefinition, VirtualColumnDefinition};
use crate::mapping::definition::table_definitions::InternalTableDefinition;

#[allow(dead_code)]
pub struct FieldStructure<'a> {
    pub is_primary_key: bool,
    pub column_attr: Option<Column>,
    pub association_column_attr: Option<AssociateColumn>,
    pub inverse_association_column_attr: Option<InverseAssociateColumn>,
    pub visibility: &'a Visibility,
    pub ident: Ident,
    pub field_type: &'a Type,
    pub resolved: bool,
    pub waiting_for: Option<String>,
    pub column_definition: Option<ColumnDefinition>,
    pub internal_column_definition: Option<InternalColumnDefinition>,
    pub virtual_column_definition: Option<VirtualColumnDefinition>,
    pub internal_table_definition: Option<InternalTableDefinition>
}