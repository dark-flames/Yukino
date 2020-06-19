use std::collections::HashMap;
use crate::mapping::definition::table_definitions::{Table};
use std::any::type_name;
use crate::entity::entities::Entity;
use std::rc::Rc;

/// Manager of definitions
/// Manage definitions in runtime
#[allow(dead_code)]
pub struct DefinitionManager {
    /// TableDefinition mapped by table name
    definitions: HashMap<String, Rc<Table>>,
    /// Table name mapped by type_table_name
    type_name_map: HashMap<&'static str, String>
}

#[allow(dead_code)]
impl DefinitionManager {
    pub fn new() -> Self {
        DefinitionManager {
            definitions: HashMap::new(),
            type_name_map: HashMap::new()
        }
    }

    pub fn register_table(&mut self, table: Rc<Table>) -> &mut Self {
        if let Some(type_name) = table.get_entity_type_name() {
            self.type_name_map.insert(
                type_name,
                table.get_name()
            );
        }

        self.definitions.insert(
            table.get_name(),
            table
        );

        self
    }

    pub fn get_definition_by_name(&self, name: &String) -> Option<Rc<Table>> {
        self.definitions.get(name).map(
            |table| Rc::clone(table)
        )
    }

    pub fn get_definition<T: 'static + Entity>(&self) -> Option<Rc<Table>> {
        let type_name = type_name::<T>();
        match self.type_name_map.get(&type_name) {
            Some(name) => self.get_definition_by_name(name),
            None => None
        }
    }
}