use crate::definitions::TableDefinition;
use crate::types::DatabaseValue;
use crate::DataConvertError;
use std::collections::HashMap;

pub trait Entity {
    fn from_database_value(
        result: &HashMap<String, DatabaseValue>,
    ) -> Result<Box<Self>, DataConvertError>;

    fn to_database_value(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;

    fn get_definitions() -> Vec<TableDefinition>;
}