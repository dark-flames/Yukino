use crate::definitions::TableDefinition;
use crate::resolver::error::DataConvertError;
use crate::types::DatabaseValue;
use std::collections::HashMap;

pub trait Entity
where
    Self: Sized,
{
    fn from_database_value(
        result: &HashMap<String, DatabaseValue>,
    ) -> Result<Box<Self>, DataConvertError>;

    fn to_database_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;

    fn get_definitions() -> Vec<TableDefinition>;

    fn primary_key_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;
}
