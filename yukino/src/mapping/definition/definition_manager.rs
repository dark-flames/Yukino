use std::collections::HashMap;
use crate::mapping::definition::table_definitions::{Table};
use std::any::TypeId;
use crate::entity::entities::Entity;
use std::rc::Rc;

/// Manager of definitions
/// Manage definitions in runtime
#[allow(dead_code)]
pub struct DefinitionManager {
    /// TableDefinition mapped by table name
    definitions: HashMap<String, Rc<Table>>,
    /// Table name mapped by
    type_id_map: HashMap<TypeId, String>
}

#[allow(dead_code)]
impl DefinitionManager {
    pub fn new() -> DefinitionManager {
        DefinitionManager {
            definitions: HashMap::new(),
            type_id_map: HashMap::new()
        }
    }

    pub fn register_table(&mut self, table: Rc<Table>) -> &mut DefinitionManager {
        if let Some(type_id) = table.get_entity_type_id() {
            self.type_id_map.insert(
                type_id,
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
        let type_id = TypeId::of::<T>();
        match self.type_id_map.get(&type_id) {
            Some(name) => self.get_definition_by_name(name),
            None => None
        }
    }
}