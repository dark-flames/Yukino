use std::collections::HashMap;
use crate::mapping::r#type::DatabaseValue;
use crate::mapping::resolver::error::ResolveError;

/// Trait of entity
/// Entity struct will implement it auto
/// Body of functions will be generated in compile time
pub trait Entity {
    fn from_raw_result(result: &HashMap<String, DatabaseValue>) -> Result<Box<Self>, ResolveError>;

    fn to_raw_value(&self) -> HashMap<String, DatabaseValue>;
}