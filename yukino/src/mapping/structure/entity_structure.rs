use crate::mapping::attribution::Table;
use syn::{Visibility, DeriveInput, Data, Error, Fields};
use proc_macro2::Ident;
use std::collections::HashMap;
use crate::mapping::structure::field_structure::FieldStructure;
use crate::mapping::definition::table_definitions::NormalTableDefinition;
use yui::AttributeStructures;

#[allow(dead_code)]
pub struct EntityStructure {
    pub table_attr: Table,
    pub visibility: Visibility,
    pub ident: Ident,
    pub fields: HashMap<String, Box<FieldStructure>>,
    pub resolved: bool,
    pub waiting_for: Option<String>,
    pub table_definition: Option<NormalTableDefinition>
}

impl EntityStructure {
    #[allow(dead_code)]
    pub fn from_ast(input: DeriveInput) -> Result<Self, Error> {
        let table_attr = AttributeStructures::from_derive_input(
            &input
        )?.attrs.into_iter().next().unwrap_or(Table {
            name: None,
            indexes: None
        });

        let fields = match input.data {
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

        let result = EntityStructure {
            table_attr,
            visibility: input.vis.clone(),
            ident: input.ident.clone(),
            fields,
            resolved: false,
            waiting_for: None,
            table_definition: None
        };

        // todo: resolve

        Ok(result)
    }
}