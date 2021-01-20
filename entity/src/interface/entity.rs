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

pub(crate) trait EntityProxy<E: Entity> {
    fn from_database_value(
        result: &HashMap<String, DatabaseValue>,
    ) -> Result<Box<Self>, DataConvertError>;

    fn unique_id(&self) -> Option<EntityUniqueID>;

    fn set_repository(&mut self, repo: &Repository<E>);

    fn get_repository(&self) -> &Repository<E>;

    fn create(inside: E) -> Self
    where
        Self: Sized;

    fn unwrap(self) -> E;

    fn drop(&mut self) {
        if let Some(id) = self.unique_id() {
            self.get_repository().pool_mut().drop_entity(&id);
        }
    }
}
