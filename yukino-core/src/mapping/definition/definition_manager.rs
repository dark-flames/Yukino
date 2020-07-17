use super::definitions::TableDefinition;
use crate::query::QueryError;
use crate::Entity;
use std::any::type_name;
use std::collections::HashMap;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Default)]
pub struct DefinitionManager {
    definitions: HashMap<String, Arc<TableDefinition>>,
    entity_definitions: HashMap<&'static str, Vec<String>>,
}

#[allow(dead_code)]
impl DefinitionManager {
    pub fn register<T: Entity>(&mut self) -> Result<&mut Self, QueryError> {
        let type_name = type_name::<T>();
        let definitions = T::get_definitions();

        let mut entity_definitions = Vec::new();

        for definition in definitions {
            entity_definitions.push(definition.name.clone());

            if self.definitions.contains_key(&definition.name) {
                return Err(QueryError::ExistedTableName(definition.name));
            }

            self.definitions
                .insert(definition.name.clone(), Arc::new(definition));
        }

        self.entity_definitions
            .insert(type_name, entity_definitions);

        Ok(self)
    }

    pub fn get_entity_definitions<T: Entity>(&self) -> Option<Vec<Arc<TableDefinition>>> {
        let type_name = type_name::<T>();
        self.entity_definitions.get(type_name).map(|names| {
            names
                .iter()
                .map(|name| Arc::clone(self.definitions.get(name).unwrap()))
                .collect()
        })
    }

    pub fn get_definition(&self, table_name: &str) -> Option<Arc<TableDefinition>> {
        self.definitions.get(table_name).map(Arc::clone)
    }
}

#[macro_export]
macro_rules! construct_definition_manager {
    ($($entity: path),*) => {
        #[macro_use]
        extern crate lazy_static;

        lazy_static! {
            static ref DEFINITION_MANAGER: yukino::mapping::DefinitionManager = {
                let mut manager = yukino::mapping::DefinitionManager::default();

                $(
                    manager.register::<$entity>();
                )*

                manager
            }
        }
    }
}
