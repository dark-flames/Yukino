use std::collections::HashMap;
use super::definitions::TableDefinition;
use super::error::DefinitionError;

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

        Ok(self.add_definitions(definitions))
    }

    pub fn add_definitions(&mut self, definitions: Vec<TableDefinition>) -> &mut Self {
        for definition in definitions {
            self.definitions.insert(definition.name.clone(), definition);
        };
        self
    }

    pub fn get(&self, name: &str) -> Option<&TableDefinition> {
        self.definitions.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut TableDefinition> {
        self.definitions.get_mut(name)
    }
}