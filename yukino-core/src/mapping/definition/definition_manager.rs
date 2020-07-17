use super::definitions::TableDefinition;
use crate::Entity;
use std::any::type_name;
use std::collections::HashMap;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Default)]
pub struct DefinitionManager {
    definitions: HashMap<&'static str, Arc<Vec<TableDefinition>>>,
}

#[allow(dead_code)]
impl DefinitionManager {
    pub fn register<T: Entity>(&mut self) -> &mut Self {
        let type_name = type_name::<T>();
        self.definitions
            .insert(type_name, Arc::new(T::get_definitions()));

        self
    }

    pub fn get_definition<T: Entity>(&self) -> Option<Arc<Vec<TableDefinition>>> {
        let type_name = type_name::<T>();
        self.definitions.get(type_name).map(Arc::clone)
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
