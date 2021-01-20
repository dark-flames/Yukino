use crate::definitions::TableDefinition;
use crate::repository::Repository;
use crate::resolver::error::DataConvertError;
use crate::types::DatabaseValue;
use std::collections::HashMap;

pub type EntityUniqueID = usize;

pub trait Entity {
    fn from_database_value(
        result: &HashMap<String, DatabaseValue>,
    ) -> Result<Box<Self>, DataConvertError>;

    fn to_database_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;

    fn get_definitions() -> Vec<TableDefinition>;

    fn primary_key_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;
}

pub trait EntityProxy<'r, E: 'r + Entity> {
    fn unique_id(&self) -> Option<EntityUniqueID>;

    fn set_unique_id(&mut self, unique_id: EntityUniqueID);

    fn get_repository(&self) -> &'r Repository<E>;

    fn create(inner: E, repo: &'r Repository<E>) -> Self
    where
        Self: Sized;

    fn unwrap(self) -> E;

    fn drop(&mut self) {
        if let Some(id) = self.unique_id() {
            self.get_repository().pool_mut().drop_entity(&id);
        }
    }
}
