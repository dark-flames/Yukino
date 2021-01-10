use crate::annotations::Entity;
use crate::resolver::error::ResolveError;
use crate::resolver::{EntityResolveStatus, EntityResolver, FieldResolver};
use annotation_rs::AnnotationStructure;
use proc_macro2::Ident;
use std::collections::HashMap;
use syn::{Data, DeriveInput, Error as SynError, Fields};

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
        let entity_annotation = match input
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path == Entity::get_path() {
                    Some(attr.parse_meta().and_then(|meta| Entity::from_meta(&meta)))
                } else {
                    None
                }
            })
            .next()
        {
            Some(annotation) => Some(annotation?),
            None => None,
        };

        if let Data::Struct(data_struct) = &input.data {
            if let Fields::Named(_) = &data_struct.fields {
                self.append_entity_resolver(
                    input.ident.clone(),
                    mod_path,
                    data_struct.fields.len(),
                    entity_annotation,
                )?;

                Ok(())
            } else {
                Err(ResolveError::UnsupportedEntityStructType.into_syn_error(&input))
            }
        } else {
            Err(ResolveError::UnsupportedEntityStructure.into_syn_error(&input))
        }
    }

    fn execute_field_resolver(&mut self, _path: &FieldPath) {}

    fn update_entity_resolver_status(&mut self, name: EntityPath) -> Result<(), SynError> {
        let resolver = self
            .entity_resolver
            .get(&name)
            .ok_or_else(|| ResolveError::EntityResolverNotFound(name.clone()).into_syn_error(""))?;

        if let EntityResolveStatus::Finished = resolver.status() {
            let empty = vec![];

            let paths = self
                .waiting_entity
                .get(&name)
                .unwrap_or_else(|| empty.as_ref())
                .clone();

            for path in paths.iter() {
                self.execute_field_resolver(path)
            }
        }

        Ok(())
    }

    fn append_entity_resolver(
        &mut self,
        ident: Ident,
        mod_path: &'static str,
        field_count: usize,
        annotation: Option<Entity>,
    ) -> Result<(), SynError> {
        let resolver = EntityResolver::new(ident, mod_path, field_count, annotation);
        let entity_name = resolver.entity_name();

        self.entity_resolver
            .insert(resolver.entity_name(), resolver);

        self.update_entity_resolver_status(entity_name)
    }
}
