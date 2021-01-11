use crate::annotations::Entity;
use crate::resolver::error::ResolveError;
use crate::resolver::{EntityResolveStatus, EntityResolver, FieldResolver, FieldResolverStatus};
use annotation_rs::AnnotationStructure;
use proc_macro2::Ident;
use std::collections::HashMap;
use syn::{Data, DeriveInput, Error as SynError, Fields};

pub type EntityPath = String;
pub type FieldName = String;
pub type FieldPath = (EntityPath, FieldName);

pub(crate) type ResolverBox = Box<dyn FieldResolver>;

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
            if let Fields::Named(_named_fields) = &data_struct.fields {
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

    fn get_field_resolver(&self, field_path: &FieldPath) -> Result<&ResolverBox, ResolveError> {
        self.field_resolver
            .get(&field_path.0)
            .map(|map| map.get(&field_path.1))
            .flatten()
            .or_else(|| {
                self.entity_resolver
                    .get(&field_path.0)
                    .map(|entity_resolver| entity_resolver.get_field_resolver(&field_path.1).ok())
                    .flatten()
            })
            .ok_or_else(|| {
                ResolveError::FieldResolverNotFound(field_path.0.clone(), field_path.1.clone())
            })
    }

    fn get_entity_resolver(&self, entity_path: &str) -> Result<&EntityResolver, ResolveError> {
        self.entity_resolver
            .get(entity_path)
            .ok_or_else(|| ResolveError::EntityResolverNotFound(entity_path.to_string()))
    }

    fn get_entity_resolver_mut(
        &mut self,
        entity_path: &str,
    ) -> Result<&mut EntityResolver, ResolveError> {
        self.entity_resolver
            .get_mut(entity_path)
            .ok_or_else(|| ResolveError::EntityResolverNotFound(entity_path.to_string()))
    }

    fn remove_field_resolver(
        &mut self,
        field_path: &FieldPath,
    ) -> Result<ResolverBox, ResolveError> {
        let field_map = self.field_resolver.get_mut(&field_path.0).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(field_path.0.clone(), field_path.1.clone())
        })?;

        field_map.remove(&field_path.1).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(field_path.0.clone(), field_path.1.clone())
        })
    }

    fn try_to_resolve_by_fields(
        &mut self,
        field_path: &FieldPath,
    ) -> Result<Option<FieldResolverStatus>, ResolveError> {
        let mut resolver = self.remove_field_resolver(field_path)?;

        if let FieldResolverStatus::WaitingForFields(fields) = resolver.status() {
            let resolvers = fields
                .iter()
                .map(|path| self.get_field_resolver(path))
                .collect::<Result<Vec<&ResolverBox>, ResolveError>>()?;

            if resolvers.iter().all(|item| item.status().is_finished()) {
                resolver.resolve_by_waiting_fields(resolvers).map(Some)
            } else {
                let paths: Vec<_> = resolvers
                    .into_iter()
                    .filter_map(|item| {
                        if !item.status().is_finished() {
                            Some(item.field_path())
                        } else {
                            None
                        }
                    })
                    .collect();
                for path in paths {
                    if let Some(list) = self.waiting_fields.get_mut(&path) {
                        list.push(field_path.clone())
                    } else {
                        let list = vec![field_path.clone()];
                        self.waiting_fields.insert(path, list);
                    }
                }

                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn update_entity_resolver_status(
        &mut self,
        entity_path: &str,
        status: EntityResolveStatus,
    ) -> Result<(), ResolveError> {
        if let EntityResolveStatus::Finished = status {
            let empty = vec![];

            let paths = self
                .waiting_entity
                .get(entity_path)
                .unwrap_or_else(|| empty.as_ref())
                .clone();

            for field_path in paths.iter() {
                let mut resolver = self.remove_field_resolver(field_path)?;

                let entity_resolver = self.get_entity_resolver(&entity_path)?;

                resolver.resolve_by_waiting_entity(entity_resolver)?;

                let status = resolver.status();

                self.append_field_resolver(resolver);

                self.update_field_resolver_status(field_path, status)?;
            }
        }

        Ok(())
    }

    fn update_field_resolver_status(
        &mut self,
        field_path: &FieldPath,
        status: FieldResolverStatus,
    ) -> Result<(), ResolveError> {
        if let Some(new_status) = match status {
            FieldResolverStatus::Finished => {
                let default = Vec::new();
                let waiting_list = self.waiting_fields.remove(&field_path).unwrap_or(default);

                for field in waiting_list {
                    self.try_to_resolve_by_fields(&field)?;
                }

                let resolver = self.remove_field_resolver(field_path)?;
                let entity_path = resolver.entity_path();

                if let EntityResolveStatus::Finished = self
                    .get_entity_resolver_mut(&entity_path)?
                    .assemble_field(resolver)?
                {
                    self.update_entity_resolver_status(
                        &entity_path,
                        EntityResolveStatus::Finished,
                    )?;
                }

                None
            }
            FieldResolverStatus::WaitingAssemble => {
                let mut resolver = self.remove_field_resolver(field_path)?;
                let entity_path = resolver.entity_path();
                let entity_resolver = self.get_entity_resolver(&entity_path)?;

                match resolver.assemble(entity_resolver) {
                    Ok(status) if !status.is_finished() => {
                        Err(ResolveError::FieldResolverNotFound(
                            field_path.0.clone(),
                            field_path.1.clone(),
                        ))
                    }
                    Ok(_) => Ok(()),
                    e => e.map(|_| ()),
                }?;

                let result = resolver.status();

                self.append_field_resolver(resolver);

                Some(result)
            }
            FieldResolverStatus::WaitingForEntity(entity_path) => {
                let mut resolver = self.remove_field_resolver(field_path)?;
                let entity_resolver = self.get_entity_resolver(&entity_path)?;

                let result = if let EntityResolveStatus::Finished = entity_resolver.status() {
                    Some(resolver.resolve_by_waiting_entity(entity_resolver)?)
                } else {
                    if let Some(fields) = self.waiting_entity.get_mut(&entity_path) {
                        fields.push(field_path.clone())
                    } else {
                        let fields = vec![field_path.clone()];
                        self.waiting_entity.insert(entity_path, fields);
                    }

                    None
                };

                self.append_field_resolver(resolver);

                result
            }
            FieldResolverStatus::WaitingForFields(_) => {
                self.try_to_resolve_by_fields(field_path)?
            }
            FieldResolverStatus::Seed => return Err(ResolveError::FieldResolverStillSeed),
        } {
            self.update_field_resolver_status(&field_path, new_status)?;
        }

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

        self.update_entity_resolver_status(&entity_name, status)
    }

    fn append_field_resolver(&mut self, resolver: ResolverBox) {
        let field_path = resolver.field_path();

        if let Some(map) = self.field_resolver.get_mut(&field_path.0) {
            map.insert(field_path.1.clone(), resolver);
        } else {
            let mut map = HashMap::new();
            map.insert(field_path.1.clone(), resolver);
            self.field_resolver.insert(field_path.0.clone(), map);
        };
    }
}
