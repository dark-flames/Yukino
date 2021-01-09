use crate::resolver::{EntityResolver, FieldResolver};
use std::collections::HashMap;
use syn::{DeriveInput, Error as SynError, Data, Fields};
use crate::annotations::Entity;
use annotation_rs::AnnotationStructure;

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

    pub fn parse(&mut self, input: DeriveInput, mod_path: &'static str) -> Result<(), SynError> {
        let entity_annotation = match input.attrs.iter().filter_map(
            |attr| if attr.path == Entity::get_path() {
                Some(attr.parse_meta().and_then(
                    |meta| Entity::from_meta(&meta)
                ))
            } else {
                None
            }
        ).next() {
            Some(annotation) => Some(annotation?),
            None => None
        };

        if let Data::Struct(data_struct) = &input.data {
            if let Fields::Named(_) = &data_struct.fields {
                let entity_resolver = EntityResolver::new(
                    input.ident.clone(),
                    mod_path,
                    data_struct.fields.len(),
                    entity_annotation
                );
                self.append_entity_resolver(entity_resolver);
                Ok(())
            } else {
                Err(SynError::new_spanned(
                    &input,
                    "Field of struct must be named field",
                ))
            }
        } else {
            Err(SynError::new_spanned(
                &input,
                "Enum or Union are not supported",
            ))
        }
    }

    fn append_entity_resolver(&mut self, _resolver: EntityResolver) {

    }
}
