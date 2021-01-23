use crate::definitions::TableDefinition;
use crate::repository::Repository;
use crate::resolver::error::DataConvertError;
use crate::types::DatabaseValue;
use std::collections::HashMap;

pub type EntityUniqueID = usize;

pub trait Entity<'t>
where
    Self: 't + Clone,
{
    type Proxy: EntityProxy<'t, Self>;

    fn from_database_value(
        result: &HashMap<String, DatabaseValue>,
    ) -> Result<Self, DataConvertError>
    where
        Self: 't + Sized;

    fn to_database_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;

    fn get_definitions() -> Vec<TableDefinition>;

    fn primary_key_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;
}

pub trait EntityProxy<'t, E: 't + Entity<'t> + Clone> {
    fn unique_id(&self) -> Option<EntityUniqueID>;

    fn set_unique_id(&mut self, unique_id: EntityUniqueID);

    fn get_repository(&self) -> &'t Repository<'t, E>
    where
        Self: Sized;

    fn create_proxy(inner: E, repo: &'t Repository<'t, E>) -> Self
    where
        Self: Sized;

    fn drop_from_pool(&mut self)
    where
        Self: 't + Sized,
    {
        if let Some(id) = self.unique_id() {
            self.get_repository().drop_entity(&id);
        }
    }

    fn inner(&self) -> E;
}
