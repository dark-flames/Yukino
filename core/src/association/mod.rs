mod associated_entity;

pub use associated_entity::*;
use crate::Entity;
use crate::types::ValuePack;
use crate::resolver::error::DataConvertError;
use crate::definitions::TableDefinition;

#[derive(Clone)]
struct FakeEntity;

impl Entity for FakeEntity {
    fn from_database_value(_result: &ValuePack) -> Result<Self, DataConvertError> where
        Self: Sized {
        unreachable!()
    }

    fn to_database_values(&self) -> Result<ValuePack, DataConvertError> {
        unreachable!()
    }

    fn get_definitions() -> Vec<TableDefinition> {
        unreachable!()
    }

    fn primary_key_values(&self) -> Result<ValuePack, DataConvertError> {
        unreachable!()
    }
}
