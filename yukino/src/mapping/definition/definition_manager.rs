use crate::mapping::definition::definitions::TableDefinition;
use std::collections::HashMap;
use crate::mapping::definition::error::DefinitionError;

#[allow(dead_code)]
pub trait DefinitionProvider {
    fn get_definitions() -> Result<Vec<TableDefinition>, DefinitionError>;
}

#[allow(dead_code)]
pub struct DefinitionManager {
    definitions: HashMap<String, TableDefinition>
}

#[allow(dead_code)]
impl DefinitionManager {
    pub fn use_provider<T: DefinitionProvider>(&mut self) -> Result<&mut Self, DefinitionError> {
        let definitions = T::get_definitions()?;

        for definition in definitions {
            self.definitions.insert(definition.name.clone(), definition);
        }

        Ok(self)
    }
}