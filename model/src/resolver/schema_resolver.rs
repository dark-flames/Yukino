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
                )
                .map_err(|err| err.into_syn_error(&input))?;

                Ok(())
            } else {
                Err(ResolveError::UnsupportedEntityStructType.into_syn_error(&input))
            }
        } else {
            Err(ResolveError::UnsupportedEntityStructure.into_syn_error(&input))
        }
    }

    fn get_field_resolver_mut(
        &mut self,
        field_path: &FieldPath,
    ) -> Result<&mut ResolverBox, ResolveError> {
        self.field_resolver
            .get_mut(&field_path.0)
            .map(|map| map.get_mut(&field_path.1))
            .flatten()
            .ok_or_else(|| {
                ResolveError::FieldResolverNotFound(field_path.1.clone(), field_path.0.clone())
            })
    }

    fn remove_field_resolver(
        &mut self,
        field_path: &FieldPath,
    ) -> Result<ResolverBox, ResolveError> {
        let field_map = self.field_resolver.get_mut(&field_path.0).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(field_path.1.clone(), field_path.0.clone())
        })?;

        field_map.remove(&field_path.1).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(field_path.1.clone(), field_path.0.clone())
        })
    }

    fn update_entity_resolver_status(
        &mut self,
        entity_path: EntityPath,
        status: EntityResolveStatus,
    ) -> Result<(), ResolveError> {
        if let EntityResolveStatus::Finished = status {
            let empty = vec![];

            let paths = self
                .waiting_entity
                .get(&entity_path)
                .unwrap_or_else(|| empty.as_ref())
                .clone();

            for field_path in paths.iter() {
                let mut resolver = self.remove_field_resolver(field_path)?;

                let entity_resolver = self
                    .entity_resolver
                    .get(&entity_path)
                    .ok_or_else(|| ResolveError::EntityResolverNotFound(entity_path.clone()))?;

                resolver.resolve_by_waiting_entity(entity_resolver)?;

                self.append_field_resolver(resolver)?;
            }
        }

        Ok(())
    }

    fn update_field_resolver_status(&mut self, field_path: &FieldPath) -> Result<(), ResolveError> {
        let _resolver = self.get_field_resolver_mut(field_path)?;

        Ok(())
    }

    fn append_entity_resolver(
        &mut self,
        ident: Ident,
        mod_path: &'static str,
        field_count: usize,
        annotation: Option<Entity>,
    ) -> Result<(), ResolveError> {
        let resolver = EntityResolver::new(ident, mod_path, field_count, annotation);
        let entity_name = resolver.entity_name();
        let status = resolver.status();

        self.entity_resolver
            .insert(resolver.entity_name(), resolver);

        self.update_entity_resolver_status(entity_name, status)
    }

    fn append_field_resolver(&mut self, resolver: ResolverBox) -> Result<(), ResolveError> {
        let field_path = resolver.field_path();

        if let Some(map) = self.field_resolver.get_mut(&field_path.0) {
            map.insert(field_path.1.clone(), resolver);
        } else {
            let mut map = HashMap::new();
            map.insert(field_path.1.clone(), resolver);
            self.field_resolver.insert(field_path.0.clone(), map);
        };

        self.update_field_resolver_status(&field_path)
    }
}
