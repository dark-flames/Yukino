use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{ValueConverter, FieldResolverSeed, FieldResolverSeedBox, FieldResolverBox, EntityName, TypePathResolver, FieldName, FieldPath, FieldResolver, FieldResolverStatus, EntityResolver, AchievedFieldResolver};
use crate::types::DatabaseValue;
use crate::Entity;
use iroha::ToTokens;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::marker::PhantomData;
use syn::{Type, PathArguments, GenericArgument, TypePath};
use proc_macro2::Ident;
use crate::annotations::{FieldAnnotation, Association};
use quote::ToTokens;
use crate::definitions::ColumnDefinition;

pub enum AssociatedEntity<E>
where
    E: Entity + Clone,
{
    Unresolved(HashMap<String, DatabaseValue>),
    Resolved(E),
}

#[derive(ToTokens)]
pub struct AssociatedEntityValueConverter<E: Entity + Clone> {
    entity_name: String,
    field_name: String,
    column_map: HashMap<String, String>,
    is_primary_key: bool,
    _marker: PhantomData<E>,
}

impl<E: Entity + Clone> ValueConverter<AssociatedEntity<E>> for AssociatedEntityValueConverter<E> {
    fn to_field_value(
        &self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<AssociatedEntity<E>, DataConvertError> {
        let value_map: HashMap<String, DatabaseValue> = values
            .iter()
            .filter_map(|(name, value)| {
                if self.column_map.contains_key(name.as_str()) {
                    Some((name.clone(), value.clone()))
                } else {
                    None
                }
            })
            .collect();

        if value_map.len() == self.column_map.len() {
            Ok(AssociatedEntity::Unresolved(value_map))
        } else {
            Err(DataConvertError::UnexpectedDatabaseValueType(
                self.entity_name.clone(),
                self.field_name.clone(),
            ))
        }
    }

    fn to_database_values_by_ref(
        &self,
        value: &AssociatedEntity<E>,
    ) -> Result<HashMap<String, DatabaseValue, RandomState>, DataConvertError> {
        match value {
            AssociatedEntity::Unresolved(map) => Ok(map.clone()),
            AssociatedEntity::Resolved(entity) => {
                let associated_result = entity.to_database_values()?;

                let reverse_map: HashMap<String, String> = self
                    .column_map
                    .iter()
                    .map(|(column, associated_column)| (associated_column.clone(), column.clone()))
                    .collect();

                Ok(associated_result
                    .into_iter()
                    .filter_map(|(column, value)| {
                        if let Some(current_column_name) = reverse_map.get(&column) {
                            Some((current_column_name.clone(), value))
                        } else {
                            None
                        }
                    })
                    .collect())
            }
        }
    }

    fn primary_column_values_by_ref(
        &self,
        value: &AssociatedEntity<E>,
    ) -> Result<HashMap<String, DatabaseValue, RandomState>, DataConvertError> {
        if self.is_primary_key {
            self.to_database_values_by_ref(value)
        } else {
            Ok(HashMap::new())
        }
    }
}

pub struct AssociatedEntityFieldResolverSeed;

impl FieldResolverSeed for AssociatedEntityFieldResolverSeed {
    fn new() -> Self where
        Self: Sized {
        AssociatedEntityFieldResolverSeed
    }

    fn boxed(&self) -> FieldResolverSeedBox {
        Box::new(AssociatedEntityFieldResolverSeed)
    }

    fn try_breed(
        &self,
        entity_name: EntityName,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
        type_path_resolver: &TypePathResolver
    ) -> Option<Result<FieldResolverBox, ResolveError>> {
        let nested_type = match field_type {
            Type::Path(type_path) => {
                let full_path = type_path_resolver.get_full_path(type_path.clone());

                let last_segment = full_path.path.segments.last()?;

                if last_segment.ident == "AssociatedEntity" {
                    match &last_segment.arguments {
                        PathArguments::AngleBracketed(arguments) => {
                            match arguments.args.first() {
                                Some(GenericArgument::Type(Type::Path(nested_type_path))) => {
                                    Some(nested_type_path.clone())
                                },
                                _ => return Some(Err(
                                    ResolveError::UnexpectedFieldGeneric(
                                        entity_name,
                                        ident.to_string()
                                )))
                            }
                        },
                        _ => return Some(Err(
                            ResolveError::UnexpectedFieldGeneric(
                                entity_name,
                                ident.to_string()
                        )))
                    }
                } else {
                    None
                }
            },
            _ => None
        }?;

        let association = annotations.iter().fold(
            None,
            |carry, item| {
                if carry.is_none() {
                    match item {
                        FieldAnnotation::Association(association) => {
                            Some(association.clone())
                        },
                        _ => None
                    }
                } else {
                    carry
                }
            }
        ).unwrap_or(Association {
            mapped_by: None
        });

        Some(Ok(Box::new(
            AssociatedEntityFieldResolver {
                field_path: (entity_name, ident.to_string()),
                nested_type: nested_type.clone(),
                primary_key: Self::is_primary_key(annotations),
                association,
                status: FieldResolverStatus::WaitingForEntity(nested_type.to_token_stream().to_string()),
                columns: vec![]
            }
        )))
    }
}

#[allow(dead_code)]
pub struct AssociatedEntityFieldResolver {
    field_path: FieldPath,
    nested_type: TypePath,
    primary_key: bool,
    association: Association,
    status: FieldResolverStatus,
    columns: Vec<ColumnDefinition>
}

impl FieldResolver for AssociatedEntityFieldResolver {
    fn status(&self) -> FieldResolverStatus {
        self.status.clone()
    }

    fn field_path(&self) -> (EntityName, FieldName) {
        self.field_path.clone()
    }

    fn resolve_by_waiting_entity(
        &mut self,
        _resolver: &EntityResolver
    ) -> Result<FieldResolverStatus, ResolveError> {
        unimplemented!()
    }

    fn resolve_by_waiting_fields(
        &mut self,
        _resolvers: Vec<&AchievedFieldResolver>
    ) -> Result<FieldResolverStatus, ResolveError> {
       unreachable!()
    }

    fn assemble(&mut self, _entity_resolver: &EntityResolver) -> Result<AchievedFieldResolver, ResolveError> {
        unimplemented!()
    }
}
