use std::collections::HashMap;
use crate::mapping::structure::EntityStructure;
use crate::mapping::r#type::ValueConverter;
use syn::{Error, parse2};
use proc_macro2::TokenStream;
use crate::mapping::error::{CompileError};

pub struct StructureManager {
    structures: HashMap<String, EntityStructure>,
    wait_for: HashMap<String, Box<Vec<String>>>,
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

    pub fn add(&mut self, token_stream: TokenStream) -> Result<&Self, Error> {
        let mut structure: EntityStructure = parse2(token_stream)?;

        let name = structure.ident.to_string();

        if !structure.resolve(
            &self,
            None
        ).map_err(
            |e| Error::new_spanned(name.clone(), e.get_message())
        )? {
            for wait_for_item in structure.wait_for.iter() {
                if self.wait_for.get_mut(wait_for_item).map(
                    |list| list.push(name.clone())
                ).is_none() {
                    self.wait_for.insert(wait_for_item.clone(), Box::new(vec![name.clone()]));
                }
            };
        };

        self.structures.insert(
            name.clone(),
            structure
        );

        // trigger resolve

        Ok(self)
    }

    pub fn get(&self, name: &String) -> Option<&EntityStructure> {
        self.structures.get(name)
    }
}