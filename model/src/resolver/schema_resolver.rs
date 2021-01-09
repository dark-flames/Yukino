use crate::resolver::{EntityResolver, FieldResolver};
use std::collections::HashMap;

pub type EntityPath = String;
pub type FieldName = String;
pub type FieldPath = (EntityPath, FieldName);

type ResolverBox = Box<dyn FieldResolver>;

#[allow(dead_code)]
pub struct SchemaResolver {
    field_resolver_seeds: Vec<ResolverBox>,
    field_resolver: HashMap<EntityPath, HashMap<FieldName, ResolverBox>>,
    entity_resolver: HashMap<EntityPath, EntityResolver>,
    waiting_fields: HashMap<FieldPath, Vec<FieldPath>>,
    waiting_entity: HashMap<EntityPath, Vec<FieldPath>>,
}

impl SchemaResolver {
    pub fn new(seeds: Vec<ResolverBox>) -> Self {
        SchemaResolver {
            field_resolver_seeds: seeds,
            field_resolver: HashMap::new(),
            entity_resolver: HashMap::new(),
            waiting_fields: HashMap::new(),
            waiting_entity: HashMap::new(),
        }
    }
}
