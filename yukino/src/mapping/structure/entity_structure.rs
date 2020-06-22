use crate::mapping::attribution::Table;
use syn::{Visibility, DeriveInput, Data, Error, Fields};
use proc_macro2::Ident;
use std::collections::HashMap;
use crate::mapping::structure::field_structure::FieldStructure;
use crate::mapping::definition::table_definitions::NormalTableDefinition;
use crate::mapping::r#type::ValueConverter;
use yui::AttributeStructures;
use syn::parse::{Parse, ParseBuffer};
use heck::SnakeCase;
use crate::mapping::error::{ResolveError};

pub struct EntityStructure {
    pub table_attr: Table,
    pub visibility: Visibility,
    pub ident: Ident,
    pub table_name: String,
    pub fields: HashMap<String, FieldStructure>,
    pub foreign_keys: Vec<String>,
    pub resolved: bool,
    pub wait_for: Vec<String>,
    pub table_definition: Option<NormalTableDefinition>
}

#[allow(dead_code)]
impl EntityStructure {
    pub fn from_ast(input: DeriveInput) -> Result<Self, Error> {
        let table_attr = AttributeStructures::from_derive_input(
            &input
        )?.attrs.into_iter().next().unwrap_or(Table {
            name: None,
            indexes: None
        });

        let fields: HashMap<String, FieldStructure> = match input.data {
            Data::Struct(input_struct) => match &input_struct.fields {
                Fields::Named(named_fields) => {
                    named_fields.named.iter().map(
                        |field| {
                            FieldStructure::from_ast(
                                field
                            ).map(|field_structure| {
                                let name = field_structure.ident.to_string();
                                (name, field_structure)
                            })
                        }
                    ).collect()
                },
                _  => Err(Error::new_spanned(
                    input_struct.fields,
                    "Field of Entity must be named field."
                ))
            },
            _ => Err(Error::new_spanned(&input, "Entity must be a struct."))
        }?;

        let foreign_keys = fields.iter().filter_map(
            |(_, item)| if item.is_primary_key {
                Some(item.ident.to_string())
            } else {
                None
            }
        ).collect();

        let table_name = table_attr.name.clone().unwrap_or(
            input.ident.to_string().to_snake_case()
        );

        let result = EntityStructure {
            table_attr,
            visibility: input.vis.clone(),
            ident: input.ident.clone(),
            fields,
            foreign_keys,
            table_name,
            resolved: false,
            wait_for: Vec::new(),
            table_definition: None
        };

        Ok(result)
    }

    pub fn resolve_internal(&mut self, converter: &ValueConverter) -> Result<bool, ResolveError> {
        let mut result = self.fields.iter_mut().map(
            |(_, field)| {
                field.resolve_independent_definition(converter)
            }
        ).fold(Ok(true), |carry, item | {
            if let Ok(true) = carry {
                item
            } else {
                carry
            }
        });

        self.resolved = result.clone().unwrap_or(false);

        if !self.resolved {
            let self_dependent_fields: Vec<String> = self.fields.iter().filter_map(
                |(name, column)| {
                    match &column.wait_for {
                        Some(wait_for) if wait_for.clone() == self.ident.to_string() => Some(name.clone()),
                        _ => None
                    }
                }
            ).collect();
            let mut resolve_result = true;

            for self_dependent_field in self_dependent_fields {
                let mut column = self.fields.remove(&self_dependent_field).unwrap();
                resolve_result = column.resolve_internal_reference(converter, &self.fields)? && resolve_result;

                self.fields.insert(self_dependent_field.clone(), column);
            }

            result = Ok(resolve_result)
        }

        result
    }

    pub fn resolve(
        &mut self,
        structures: &HashMap<String, EntityStructure>,
        converter: &ValueConverter,
        new_entity_name: String
    ) -> Result<bool, ResolveError> {
        let result = self.fields.iter_mut().filter(|(_, field)| {
            field.wait_for.is_some() && field.wait_for.clone().unwrap() == new_entity_name
        }).map(|(_, field)| {
            field.resolve(converter, structures)
        }).fold(Ok(true), |result, item| {
            if let Ok(true) = item {
                result
            } else {
                item
            }
        });

        self.resolved = result.clone().unwrap_or(false);

        result
    }
}

impl Parse for EntityStructure {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let derive_input = DeriveInput::parse(input)?;

        Self::from_ast(derive_input)
    }
}