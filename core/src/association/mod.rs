mod associated_entity;

use crate::definitions::{FieldDefinition, TableDefinition};
use crate::resolver::error::DataConvertError;
use crate::types::ValuePack;
use crate::Entity;
pub use associated_entity::*;

#[derive(Clone)]
struct FakeEntity;

impl Entity for FakeEntity {
    fn from_database_value(_result: &ValuePack) -> Result<Self, DataConvertError>
    where
        Self: Sized,
    {
        unreachable!()
    }

    fn to_database_values(&self) -> Result<ValuePack, DataConvertError> {
        unreachable!()
    }

    fn get_definitions() -> Vec<TableDefinition> {
        unreachable!()
    }

    fn get_field_definition(_field_name: &str) -> Option<FieldDefinition> {
        unreachable!()
    }

    fn primary_key_values(&self) -> Result<ValuePack, DataConvertError> {
        unreachable!()
    }
}
