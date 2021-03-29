use crate::definitions::{FieldDefinition, TableDefinition};
use crate::resolver::error::DataConvertError;
use crate::transaction::Transaction;
use crate::types::ValuePack;

pub type EntityUniqueID = usize;

pub trait Entity
where
    Self: 'static + Clone,
{
    fn from_database_value(result: &ValuePack) -> Result<Self, DataConvertError>
    where
        Self: Sized;

    fn to_database_values(&self) -> Result<ValuePack, DataConvertError>;

    fn get_definitions() -> Vec<TableDefinition>;

    fn get_field_definition(field_name: &str) -> Option<FieldDefinition>;

    fn primary_key_values(&self) -> Result<ValuePack, DataConvertError>;
}

pub trait EntityProxy<'t, E: 't + Entity + Clone> {
    type Entity = E;
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
