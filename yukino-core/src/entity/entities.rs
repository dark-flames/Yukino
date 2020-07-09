use std::collections::HashMap;
use crate::entity::error::ParseError;
use crate::mapping::r#type::DatabaseValue;
use crate::mapping::definition::TableDefinition;

/// Trait of entity
/// Entity struct will implement it auto
/// Body of functions will be generated in compile time
pub trait Entity {
    fn from_raw_result(result: &HashMap<String, DatabaseValue>) -> Result<Box<Self>, ParseError>;

    fn to_raw_value(&self) -> Result<HashMap<String, DatabaseValue>, ParseError>;

    fn get_definitions(&self) -> Vec<TableDefinition>;
}