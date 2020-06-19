use super::super::attribution::{Column, Association, InverseAssociation};
use syn::{Type, Visibility, Field, Error};
use proc_macro2::Ident;
use crate::mapping::definition::column_definitions::{ColumnDefinition, InternalColumnDefinition, VirtualColumnDefinition};
use crate::mapping::definition::table_definitions::InternalTableDefinition;
use yui::AttributeStructure;
use crate::mapping::attribution::Id;
use crate::mapping::error::TypeError;
use super::super::r#type::ValueConverter;

#[allow(dead_code)]
pub struct FieldStructure {
    pub is_primary_key: bool,
    pub column_attr: Option<Column>,
    pub association_column_attr: Option<Association>,
    pub inverse_association_column_attr: Option<InverseAssociation>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub field_type: Type,
    pub resolved: bool,
    pub waiting_for: Option<String>,
    pub column_definition: Option<ColumnDefinition>,
    pub internal_column_definition: Option<InternalColumnDefinition>,
    pub virtual_column_definition: Option<VirtualColumnDefinition>,
    pub internal_table_definition: Option<InternalTableDefinition>
}

#[allow(dead_code)]
impl FieldStructure {
    pub fn from_ast(input_field: &Field) -> Result<Self, Error> {
        let mut is_primary_key = false;
        let mut column_attr = None;
        let mut association_column_attr = None;
        let mut inverse_association_column_attr = None;

        for attr in input_field.attrs.iter() {
            if attr.path == Id::get_path() {
                is_primary_key = true;
            } else if attr.path == Column::get_path() {
                column_attr = Some(Column::from_meta(&attr.parse_meta()?)?);
            } else if attr.path == Association::get_path() {
                association_column_attr = Some(
                    Association::from_meta(&attr.parse_meta()?)?
                );
            } else if attr.path == InverseAssociation::get_path(){
                inverse_association_column_attr = Some(
                    InverseAssociation::from_meta(&attr.parse_meta()?)?
                );
            }
        }

        let result = FieldStructure {
            is_primary_key,
            column_attr,
            association_column_attr,
            inverse_association_column_attr,
            visibility: input_field.vis.clone(),
            ident: input_field.ident.as_ref().unwrap().clone(),
            field_type: input_field.ty.clone(),
            resolved: false,
            waiting_for: None,
            column_definition: None,
            internal_column_definition: None,
            virtual_column_definition: None,
            internal_table_definition: None
        };

        if let Some(error_message) = result.check() {
            return Err(Error::new_spanned(&input_field, error_message))
        }

        Ok(result)
    }

    fn check(&self) -> Option<&str> {
        match [
            self.column_attr.is_some() as u8,
            self.association_column_attr.is_some() as u8,
            self.inverse_association_column_attr.is_some() as u8
        ].iter().sum() {
            1 => (),
            0 => return Some(
                "A Field of Entity must have one column attribute (Column, Association or InverseAssociation) at least."
            ),
            _ => return Some(
                "A Field of Entity can not have multiple column attribute Association or InverseAssociation)."
            )
        };

        // more check

        None
    }

    fn resolve_independent_definition(&mut self, converter: &ValueConverter) -> Result<bool, TypeError> {
        if let Some(_) = &self.column_attr {
            self.column_definition = Some(ColumnDefinition::from_structure(self, converter)?);
            self.resolved = true;
        };

        Ok(self.resolved)
    }
}