use crate::annotations::{Entity, FieldAnnotation};
use crate::definitions::TableDefinition;
use crate::resolver::error::ResolveError;
use crate::resolver::{
    AchievedEntityResolver, AchievedFieldResolver, EntityResolveStatus, EntityResolver,
    EntityResolverPassBox, FieldResolverBox, FieldResolverSeedBox, FieldResolverStatus,
    TypePathResolver,
};
use annotation_rs::AnnotationStructure;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use syn::{Error as SynError, Fields, ItemStruct};

pub type EntityName = String;
pub type FieldName = String;
pub type FieldPath = (EntityName, FieldName);

pub struct SchemaResolver {
    field_resolver_seeds: Vec<FieldResolverSeedBox>,
    field_resolver: HashMap<EntityName, HashMap<FieldName, FieldResolverBox>>,
    entity_resolver: HashMap<EntityName, EntityResolver>,
    waiting_fields: HashMap<FieldPath, Vec<FieldPath>>,
    waiting_entity: HashMap<EntityName, Vec<FieldPath>>,
    entity_resolver_passes: Vec<EntityResolverPassBox>,
}

impl SchemaResolver {
    pub fn new(
        seeds: Vec<FieldResolverSeedBox>,
        entity_resolver_passes: Vec<EntityResolverPassBox>,
    ) -> Self {
        SchemaResolver {
            field_resolver_seeds: seeds,
            field_resolver: HashMap::new(),
            entity_resolver: HashMap::new(),
            waiting_fields: HashMap::new(),
            waiting_entity: HashMap::new(),
            entity_resolver_passes,
        }
    }

    pub fn parse(
        &mut self,
        input: ItemStruct,
        type_path_resolver: &TypePathResolver,
    ) -> Result<(), SynError> {
        let entity_annotation = input
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
            .map_or(Ok(None), |v| v.map(Some))?;

        if let Fields::Named(named_fields) = &input.fields {
            let entity_name = self
                .append_entity_resolver(
                    input.ident.clone(),
                    input.fields.len(),
                    entity_annotation,
                    input.clone(),
                )
                .map_err(|err| err.into_syn_error(&input))?;

            for field in named_fields.named.iter() {
                let field_annotations = field
                    .attrs
                    .iter()
                    .map(|attr| FieldAnnotation::from_attr(attr))
                    .collect::<Result<Vec<FieldAnnotation>, SynError>>()?;
                let field_name = field.ident.as_ref().unwrap().to_string();
                let field_resolver = self
                    .field_resolver_seeds
                    .iter()
                    .fold(
                        Err(ResolveError::NoSuitableResolverSeedsFound(
                            entity_name.clone(),
                            field_name,
                        )),
                        |result, seed| {
                            if result.is_err() {
                                seed.try_breed(
                                    entity_name.clone(),
                                    field.ident.as_ref().unwrap(),
                                    &field_annotations,
                                    &field.ty,
                                    type_path_resolver,
                                )
                                .unwrap_or(result)
                            } else {
                                result
                            }
                        },
                    )
                    .map_err(|e| e.into_syn_error(field))?;
                let status = field_resolver.status();
                let field_path = field_resolver.field_path();

                self.append_field_resolver(field_resolver);
                self.update_field_resolver_status(&field_path, status)
                    .map_err(|e| e.into_syn_error(field))?;
            }

            Ok(())
        } else {
            Err(ResolveError::UnsupportedEntityStructType.into_syn_error(&input))
        }
    }

    pub fn achieve(
        self,
        type_path_resolver: &TypePathResolver,
    ) -> Result<ImmutableSchemaResolver, SynError> {
        if let Some(map) = self.field_resolver.values().next() {
            if let Some(resolver) = map.values().next() {
                let field_path = resolver.field_path();
                return Err(ResolveError::FieldResolverIsNotFinished(
                    field_path.0.clone(),
                    field_path.1,
                )
                .into_syn_error(""));
            }
        }

        let resolvers = self
            .entity_resolver
            .into_iter()
            .map(|(path, resolver)| {
                resolver
                    .achieve(type_path_resolver)
                    .map(|achieved| (path, achieved))
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.into_syn_error(""))?;

        Ok(ImmutableSchemaResolver {
            resolvers: resolvers.into_iter().collect(),
        })
    }

    fn get_achieved_field_resolver(
        &self,
        field_path: &FieldPath,
    ) -> Result<&AchievedFieldResolver, ResolveError> {
        self.entity_resolver
            .get(&field_path.0)
            .map(|entity_resolver| entity_resolver.get_field_resolver(&field_path.1).ok())
            .flatten()
            .ok_or_else(|| {
                ResolveError::FieldResolverNotFound(field_path.0.clone(), field_path.1.clone())
            })
    }

    fn get_entity_resolver(&self, entity_name: &str) -> Option<&EntityResolver> {
        self.entity_resolver.get(entity_name)
    }

    fn get_entity_resolver_mut(&mut self, entity_name: &str) -> Option<&mut EntityResolver> {
        self.entity_resolver.get_mut(entity_name)
    }

    fn remove_field_resolver(
        &mut self,
        field_path: &FieldPath,
    ) -> Result<FieldResolverBox, ResolveError> {
        let field_map = self.field_resolver.get_mut(&field_path.0).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(field_path.0.clone(), field_path.1.clone())
        })?;

        let result = field_map.remove(&field_path.1).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(field_path.0.clone(), field_path.1.clone())
        });

        if field_map.is_empty() {
            self.field_resolver.remove(&field_path.0);
        }

        result
    }

    fn try_to_resolve_by_fields(
        &mut self,
        field_path: &FieldPath,
    ) -> Result<Option<FieldResolverStatus>, ResolveError> {
        let mut resolver = self.remove_field_resolver(field_path)?;

        let result = if let FieldResolverStatus::WaitingForFields(fields) = resolver.status() {
            let resolvers: Vec<_> = fields
                .iter()
                .map(|path| (path.clone(), self.get_achieved_field_resolver(path).ok()))
                .collect();

            if resolvers.iter().any(|(_, resolver)| resolver.is_none()) {
                resolver
                    .resolve_by_waiting_fields(
                        resolvers
                            .into_iter()
                            .filter_map(|(_, resolver)| resolver)
                            .collect(),
                    )
                    .map(Some)
            } else {
                let paths: Vec<_> = resolvers
                    .into_iter()
                    .filter_map(|(name, item)| item.map(|_| name))
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
        };

        self.append_field_resolver(resolver);

        result
    }

    fn update_entity_resolver_status(
        &mut self,
        entity_name: &str,
        status: EntityResolveStatus,
    ) -> Result<(), ResolveError> {
        if EntityResolveStatus::Finished == status {
            let empty = vec![];

            let paths = self
                .waiting_entity
                .get(entity_name)
                .unwrap_or_else(|| empty.as_ref())
                .clone();

            for field_path in paths.iter() {
                let mut resolver = self.remove_field_resolver(field_path)?;

                let entity_resolver = self
                    .get_entity_resolver(&entity_name)
                    .ok_or_else(|| ResolveError::EntityResolverNotFound(entity_name.to_string()))?;

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
            FieldResolverStatus::WaitingAssemble => {
                let mut resolver = self.remove_field_resolver(field_path)?;
                let entity_name = resolver.field_path().0;
                let entity_resolver = self
                    .get_entity_resolver(&entity_name)
                    .ok_or_else(|| ResolveError::EntityResolverNotFound(entity_name.clone()))?;

                let achieved = resolver.assemble(entity_resolver)?;
                if EntityResolveStatus::Finished
                    == self
                        .get_entity_resolver_mut(&entity_name)
                        .ok_or_else(|| ResolveError::EntityResolverNotFound(entity_name.clone()))?
                        .assemble_field(achieved)?
                {
                    self.update_entity_resolver_status(
                        &entity_name,
                        EntityResolveStatus::Finished,
                    )?;
                }

                let default = Vec::new();
                let waiting_list = self.waiting_fields.remove(&field_path).unwrap_or(default);

                for field in waiting_list {
                    self.try_to_resolve_by_fields(&field)?;
                }

                None
            }
            FieldResolverStatus::WaitingForEntity(entity_name) => {
                let mut resolver = self.remove_field_resolver(field_path)?;
                let entity_resolver = self.get_entity_resolver(&entity_name);

                let result =
                    if Some(EntityResolveStatus::Finished) == entity_resolver.map(|r| r.status()) {
                        Some(resolver.resolve_by_waiting_entity(entity_resolver.unwrap())?)
                    } else {
                        if let Some(fields) = self.waiting_entity.get_mut(&entity_name) {
                            fields.push(field_path.clone())
                        } else {
                            let fields = vec![field_path.clone()];
                            self.waiting_entity.insert(entity_name, fields);
                        }

                        None
                    };

                self.append_field_resolver(resolver);

                result
            }
            FieldResolverStatus::WaitingForFields(_) => {
                self.try_to_resolve_by_fields(field_path)?
            }
        } {
            self.update_field_resolver_status(&field_path, new_status)?;
        }

        Ok(())
    }

    fn append_entity_resolver(
        &mut self,
        ident: Ident,
        field_count: usize,
        annotation: Option<Entity>,
        input: ItemStruct,
    ) -> Result<EntityName, ResolveError> {
        let resolver = EntityResolver::new(
            ident,
            field_count,
            annotation,
            self.entity_resolver_passes
                .iter()
                .map(|item| item.boxed())
                .collect(),
            input,
        );
        let entity_name = resolver.entity_name();
        let status = resolver.status();

        self.entity_resolver
            .insert(resolver.entity_name(), resolver);

        self.update_entity_resolver_status(&entity_name, status)?;

        Ok(entity_name)
    }

    fn append_field_resolver(&mut self, resolver: FieldResolverBox) {
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

pub struct ImmutableSchemaResolver {
    resolvers: HashMap<EntityName, AchievedEntityResolver>,
}

impl ImmutableSchemaResolver {
    pub fn get_definitions(&self) -> Vec<TableDefinition> {
        self.resolvers
            .values()
            .map(|resolver| resolver.definitions.clone())
            .flatten()
            .collect()
    }

    pub fn get_implements(&self) -> TokenStream {
        self.resolvers
            .values()
            .map(|resolver| &resolver.implement)
            .fold(TokenStream::new(), |mut previous, current| {
                current.to_tokens(&mut previous);

                previous
            })
    }
}
