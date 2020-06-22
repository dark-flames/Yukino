use std::collections::HashMap;
use crate::mapping::structure::EntityStructure;
use crate::mapping::r#type::ValueConverter;
use syn::{Error, parse2};
use proc_macro2::TokenStream;
use crate::mapping::error::{CompileError, ResolveError};

pub struct StructureManager {
    structures: HashMap<String, EntityStructure>,
    wait_for: HashMap<String, Vec<String>>,
    pub converter: ValueConverter
}

#[allow(dead_code)]
impl StructureManager {
    pub fn new(converter: ValueConverter) -> Self {
        StructureManager {
            structures: HashMap::new(),
            wait_for: HashMap::new(),
            converter
        }
    }

    fn add_structure(&mut self, structure: EntityStructure) {
        self.structures.insert(structure.ident.to_string(), structure);
    }

    pub fn add(&mut self, token_stream: TokenStream) -> Result<&Self, Error> {
        let mut structure: EntityStructure = parse2(token_stream)?;

        let name = structure.ident.to_string();

        let resolve_result = structure.resolve_internal(
            &self.converter
        ).map_err(
            |e| Error::new_spanned(name.clone(), e.get_message())
        )?;

        if !resolve_result {
            for wait_for_item in structure.wait_for.iter() {
                if self.wait_for.get_mut(wait_for_item).map(
                    |list| list.push(name.clone())
                ).is_none() {
                    self.wait_for.insert(wait_for_item.clone(), vec![name.clone()]);
                }
            };
        };

        self.add_structure(structure);

        // trigger resolve

        Ok(self)
    }

    fn resolve_for(
        structures: &mut HashMap<String, EntityStructure>,
        converter: &ValueConverter,
        name: String,
        new_entity_name: String,
    ) -> Result<bool, ResolveError> {
        let mut structure = structures.remove(&name).ok_or(
            ResolveError::new(&name, "Unknown entity")
        )?;

        let result = structure.resolve(
            &structures,
            converter,
            new_entity_name
        );

        structures.insert(name, structure);

        result
    }

    pub fn trigger_resolve(&mut self, new_entity_name: &String) -> Result<(), ResolveError> {
        let wait_for = &mut self.wait_for;
        let structure = &mut self.structures;
        let converter = &self.converter;

        if let Some(list) = wait_for.get_mut(new_entity_name) {
            let result = list.iter().map(
                |item| {
                    Self::resolve_for(
                        structure,
                        converter,
                        item.clone(),
                        new_entity_name.clone()
                    )
                }
            ).collect::<Result<Vec<bool>, ResolveError>>()?;

            let mut index = 0;
            list.retain(|_| {
                let remove = result[index];
                index += 1;

                !remove
            });

            Ok(())
        } else {
            Ok(())
        }
    }
}