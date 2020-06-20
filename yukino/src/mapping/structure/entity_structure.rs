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
use crate::mapping::error::{TypeError, ResolveError};
use crate::mapping::structure::structure_manager::StructureManager;

pub struct EntityStructure {
    pub table_attr: Table,
    pub visibility: Visibility,
    pub ident: Ident,
    pub table_name: String,
    pub fields: HashMap<String, Box<FieldStructure>>,
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

        let fields: HashMap<String, Box<FieldStructure>> = match input.data {
            Data::Struct(input_struct) => match &input_struct.fields {
                Fields::Named(named_fields) => {
                    named_fields.named.iter().map(
                        |field| {
                            FieldStructure::from_ast(
                                field
                            ).map(|field_structure| {
                                let name = field_structure.ident.to_string();
                                (name, Box::new(field_structure))
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

    pub fn resolve_independent_column(&mut self, converter: &ValueConverter ) -> Result<bool, TypeError> {
        let result = self.fields.iter_mut().map(
            |(_, field)| {
                field.resolve_independent_definition(converter)
            }
        ).fold(Ok(true), |carry, item | {
            match carry {
                Ok(carry_result) => match item {
                    Ok(item_result) => Ok(item_result && carry_result),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            }
        });

        match result {
            Ok(true) => self.resolved = true,
            _ => ()
        }

        result
    }

    pub fn resolve(&mut self, manager: &StructureManager, new_entity_name: Option<&String>) -> Result<bool, ResolveError> {
        &manager;
        &new_entity_name;
        // todo: fix me
        Ok(false)
    }
}

impl Parse for EntityStructure {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let derive_input = DeriveInput::parse(input)?;

        Self::from_ast(derive_input)
    }
}