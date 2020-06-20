use super::super::attribution::{Column, Association, InverseAssociation};
use syn::{Type, Visibility, Field, Error};
use proc_macro2::Ident;
use crate::mapping::definition::column_definitions::{ColumnDefinition, InternalColumnDefinition, VirtualColumnDefinition};
use crate::mapping::definition::table_definitions::InternalTableDefinition;
use yui::AttributeStructure;
use crate::mapping::attribution::Id;
use crate::mapping::error::{TypeError, ResolveError, CompileError};
use super::super::r#type::ValueConverter;
use crate::mapping::structure::structure_manager::StructureManager;
use crate::mapping::structure::{unwrap_association_type, AssociationFieldType};
use heck::SnakeCase;

#[allow(dead_code)]
pub struct FieldStructure {
    pub is_primary_key: bool,
    pub column_attr: Option<Column>,
    pub association_column_attr: Option<Association>,
    pub inverse_association_column_attr: Option<InverseAssociation>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub field_type: Type,
    pub association_field_type: Option<AssociationFieldType>,
    pub resolved: bool,
    pub wait_for: Option<String>,
    pub column_definition: Option<ColumnDefinition>,
    pub internal_column_definition: Option<Vec<InternalColumnDefinition>>,
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
        };

        let association_field_type = if association_column_attr.is_some() || inverse_association_column_attr.is_some() {
            Some(unwrap_association_type(
                &input_field.ty
            ).map_err(
                |e| Error::new_spanned(&input_field, e.get_message())
            )?)
        } else {
            None
        };

        let result = FieldStructure {
            is_primary_key,
            column_attr,
            association_column_attr,
            inverse_association_column_attr,
            visibility: input_field.vis.clone(),
            ident: input_field.ident.as_ref().unwrap().clone(),
            field_type: input_field.ty.clone(),
            association_field_type,
            resolved: false,
            wait_for: None,
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

    pub fn resolve_independent_definition(&mut self, converter: &ValueConverter) -> Result<bool, TypeError> {
        if let Some(_) = &self.column_attr {
            self.column_definition = Some(ColumnDefinition::from_structure(self, converter)?);
            self.resolved = true;
        };

        Ok(self.resolved)
    }

    pub fn resolve(&mut self, manager: &StructureManager) -> Result<bool, ResolveError> {
        if self.column_attr.is_some() {
            self.column_definition = Some(
                ColumnDefinition::from_structure(self, &manager.converter)?);
            self.resolved = true;
        } else if let Some(attr) = &self.association_column_attr {
            let target_entity = self.association_field_type.as_ref().unwrap().get_type();
            let target = match manager.get(&target_entity) {
                Some(s) => s,
                None => {
                    self.wait_for = Some(target_entity);
                    return Ok(false)
                }
            };
            let name = self.ident.to_string().to_snake_case();
            let target_name = target.ident.to_string().to_snake_case();

            let columns = match &attr.mapped_by {
                Some(fields) if !fields.is_empty() => {
                    Ok(fields)
                },
                None if !target.foreign_keys.is_empty() => {
                    Ok(&target.foreign_keys)
                },
                _ => Err(ResolveError::new(
                    &self.ident,
                    "'mapped_by' fields must be determined if associated Entity is using auto primary key"
                ))
            }?.iter().map(
                |field_name| {
                    match target.fields.get(
                        field_name
                    ).ok_or(
                        ResolveError::new(
                            &name,
                            &format!("Unknown field '{}' found in mapped_by", field_name)
                        )
                    ).map(|field_structure| {
                        let reference_definition = match field_structure.column_definition.as_ref().ok_or(
                            ResolveError::new(
                                &name,
                                "Associated column cannot be an association column"
                            )
                        ) {
                            Ok(d) => d,
                            Err(e) => return Err(e)
                        };

                        Ok(InternalColumnDefinition {
                            name: format!(
                                "{}_{}_{}",
                                name,
                                target_name,
                                field_name.to_snake_case()
                            ),
                            column_type: reference_definition.column_type.clone(),
                            logic_type: reference_definition.logic_type.clone(),
                            reference_table: target.table_name.clone(),
                            reference_column: field_name.clone()
                        })
                    }) {
                        Ok(Ok(result)) => Ok(result),
                        Ok(Err(e)) => Err(e),
                        Err(e) => Err(e)
                    }
                }
            ).collect::<Result<Vec<InternalColumnDefinition>, ResolveError>>()?;

            self.internal_column_definition = Some(columns);
        } else if self.inverse_association_column_attr.is_some() {

        }

        Ok(self.resolved)
    }
}