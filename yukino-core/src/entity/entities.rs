use crate::entity::error::ParseError;
use crate::mapping::definition::TableDefinition;
use crate::mapping::r#type::DatabaseValue;
use std::collections::HashMap;

/// Trait of entity
/// Entity struct will implement it auto
/// Body of functions will be generated in compile time
pub trait Entity {
    fn from_database_value(
        result: &HashMap<String, DatabaseValue>,
    ) -> Result<Box<Self>, ParseError>;

    fn to_database_value(&self) -> Result<HashMap<String, DatabaseValue>, ParseError>;

    fn get_definitions(&self) -> Vec<TableDefinition>;
}
