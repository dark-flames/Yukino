use crate::definitions::TableDefinition;
use crate::resolver::error::DataConvertError;
use crate::transaction::Transaction;
use crate::types::DatabaseValue;
use std::collections::HashMap;

pub type EntityUniqueID = usize;

pub trait Entity
where
    Self: 'static + Clone,
{
    fn from_database_value(
        result: &HashMap<String, DatabaseValue>,
    ) -> Result<Self, DataConvertError>
    where
        Self: Sized;

    fn to_database_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;

    fn get_definitions() -> Vec<TableDefinition>;

    fn primary_key_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;
}

pub trait EntityProxy<'t, E: 't + Entity + Clone> {
    fn unique_id(&self) -> Option<EntityUniqueID>;

    fn set_unique_id(&mut self, unique_id: EntityUniqueID);

    fn get_transaction(&self) -> &'t Transaction
    where
        Self: Sized;

    fn create_proxy(inner: E, transaction: &'t Transaction) -> Self
    where
        Self: Sized;

    fn drop_from_pool(&mut self)
    where
        Self: Sized,
    {
        if let Some(id) = self.unique_id() {
            self.get_transaction()
                .get_repository::<E>()
                .drop_entity(&id);
        }
    }

    fn inner(&self) -> E;
}
