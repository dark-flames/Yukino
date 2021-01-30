use crate::annotations::{Association, Field, FieldAnnotation, IndexMethod};
use crate::association::FakeEntity;
use crate::definitions::{ColumnDefinition, ColumnType, ForeignKeyDefinition, IndexDefinition};
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{
    AchievedFieldResolver, EntityName, EntityResolver, FieldName, FieldPath, FieldResolver,
    FieldResolverBox, FieldResolverSeed, FieldResolverSeedBox, FieldResolverStatus,
    TypePathResolver, ValueConverter,
};
use crate::types::DatabaseValue;
use crate::Entity;
use heck::SnakeCase;
use iroha::ToTokens;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::str::FromStr;
use syn::{GenericArgument, PathArguments, Type};

pub enum AssociatedEntity<E>
where
    E: Entity + Clone,
{
    Unresolved(HashMap<String, DatabaseValue>),
    Resolved(E),
}

impl<E> AssociatedEntity<E>
where
    E: Entity + Clone,
{
    pub fn resolved(&self) -> bool {
        matches!(self, Self::Resolved(_))
    }

    pub fn get(&self) -> Option<&E> {
        match self {
            Self::Resolved(entity) => Some(entity),
            _ => None,
        }
    }
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
    fn new() -> Self
    where
        Self: Sized,
    {
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
        type_path_resolver: &TypePathResolver,
    ) -> Option<Result<FieldResolverBox, ResolveError>> {
        let nested_type = match field_type {
            Type::Path(type_path) => {
                let full_path = type_path_resolver.get_full_path(type_path.clone());

                let last_segment = full_path.path.segments.last()?;

                if last_segment.ident == "AssociatedEntity" {
                    match &last_segment.arguments {
                        PathArguments::AngleBracketed(arguments) => match arguments.args.first() {
                            Some(GenericArgument::Type(Type::Path(nested_type_path))) => {
                                Some(nested_type_path.clone())
                            }
                            _ => {
                                return Some(Err(ResolveError::UnexpectedFieldGeneric(
                                    entity_name,
                                    ident.to_string(),
                                )))
                            }
                        },
                        _ => {
                            return Some(Err(ResolveError::UnexpectedFieldGeneric(
                                entity_name,
                                ident.to_string(),
                            )))
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }?;

        let association = annotations
            .iter()
            .fold(None, |carry, item| {
                if carry.is_none() {
                    match item {
                        FieldAnnotation::Association(association) => Some(association.clone()),
                        _ => None,
                    }
                } else {
                    carry
                }
            })
            .unwrap_or(Association { mapped_by: None });

        Some(Ok(Box::new(AssociatedEntityFieldResolver {
            field_path: (entity_name, ident.to_string()),
            field_type: field_type.clone(),
            nested_type: Type::Path(nested_type.clone()),
            primary_key: Self::is_primary_key(annotations),
            association,
            field_annotation: Self::default_annotations(annotations),
            status: FieldResolverStatus::WaitingForEntity(
                nested_type.to_token_stream().to_string(),
            ),
            referenced_table: None,
            columns: vec![],
            column_map: vec![],
        })))
    }
}

#[allow(dead_code)]
pub struct AssociatedEntityFieldResolver {
    field_path: FieldPath,
    field_type: Type,
    nested_type: Type,
    primary_key: bool,
    association: Association,
    field_annotation: Field,
    status: FieldResolverStatus,
    referenced_table: Option<String>,
    columns: Vec<ColumnDefinition>,
    column_map: Vec<(String, String)>,
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
        resolver: &EntityResolver,
    ) -> Result<FieldResolverStatus, ResolveError> {
        match self.status.clone() {
            FieldResolverStatus::WaitingForEntity(waited_entity) => {
                assert_eq!(waited_entity, resolver.entity_name());

                let mapped_by = match &self.association.mapped_by {
                    Some(columns) => columns.clone(),
                    _ => resolver.get_primary_columns()?,
                };

                if !resolver.is_unique_fields(&mapped_by)? {
                    return Err(ResolveError::MappingFieldsNotUnique(
                        self.field_path.0.clone(),
                        self.field_path.1.clone(),
                    ));
                }

                self.referenced_table = Some(resolver.table_name());

                self.column_map = mapped_by
                    .iter()
                    .map(
                        |referenced_field_name| -> Result<Vec<(String, String)>, ResolveError> {
                            Ok(resolver
                                .get_field_resolver(&referenced_field_name)?
                                .columns
                                .iter()
                                .map(|definition| {
                                    (
                                        format!(
                                            "{}_{}",
                                            self.field_path.1.to_snake_case(),
                                            definition.name
                                        ),
                                        definition.name.clone(),
                                    )
                                })
                                .collect())
                        },
                    )
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .fold(vec![], |mut carry, mut item| {
                        carry.append(&mut item);
                        carry
                    });

                self.columns = mapped_by
                    .iter()
                    .map(
                        |referenced_field_name| -> Result<Vec<ColumnDefinition>, ResolveError> {
                            Ok(resolver
                                .get_field_resolver(referenced_field_name)?
                                .columns
                                .iter()
                                .map(|definition| ColumnDefinition {
                                    name: format!(
                                        "{}_{}",
                                        self.field_path.1.to_snake_case(),
                                        definition.name
                                    ),
                                    ty: ColumnType::VisualColumn,
                                    data_type: definition.data_type,
                                    unique: false,
                                    auto_increase: false,
                                    primary_key: self.primary_key,
                                })
                                .collect())
                        },
                    )
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .fold(vec![], |mut carry, mut item| {
                        carry.append(&mut item);
                        carry
                    });

                Ok(FieldResolverStatus::WaitingAssemble)
            }
            s => Err(ResolveError::UnexpectedFieldResolverStatus(
                self.field_path.0.clone(),
                self.field_path.1.clone(),
                "WaitingForEntity".to_string(),
                s,
            )),
        }
    }

    fn resolve_by_waiting_fields(
        &mut self,
        _resolvers: Vec<&AchievedFieldResolver>,
    ) -> Result<FieldResolverStatus, ResolveError> {
        unreachable!()
    }

    fn assemble(
        &mut self,
        _entity_resolver: &EntityResolver,
    ) -> Result<AchievedFieldResolver, ResolveError> {
        if let FieldResolverStatus::WaitingAssemble = self.status() {
            let field_snake_case = self.field_path.1.to_snake_case();
            let indexes = if self.field_annotation.unique {
                vec![IndexDefinition {
                    name: format!("{}_unique", field_snake_case),
                    columns: self
                        .columns
                        .iter()
                        .map(|definition| definition.name.clone())
                        .collect(),
                    method: IndexMethod::BTree,
                    unique: true,
                }]
            } else {
                vec![]
            };

            let converter_getter_name =
                quote::format_ident!("get_{}_converter", &self.field_path().1.to_snake_case());

            let convert = AssociatedEntityValueConverter {
                entity_name: self.field_path.0.clone(),
                field_name: self.field_path.1.clone(),
                column_map: self.column_map.iter().cloned().collect(),
                is_primary_key: self.primary_key,
                _marker: PhantomData::<FakeEntity>::default(),
            };

            let entity_ident = format_ident!("{}Inner", self.field_path.0);

            let output_type = quote! {
                yukino::resolver::field_resolver_seeds::AssociatedEntityValueConverter<#entity_ident>
            };

            let data_converter_token_stream = quote! {
                pub fn #converter_getter_name() -> #output_type {
                    #output_type::new(
                        #convert
                    )
                }
            };

            let getter_name = self.getter_ident();
            let setter_name = self.setter_ident();

            let field_ident = TokenStream::from_str(self.field_path.1.as_str()).unwrap();

            //todo: nullable

            let nested_type = &self.nested_type;
            let field_type = &self.field_type;

            let field_getter_token_stream = quote! {
                pub fn #getter_name(&self) -> &#field_type {
                    let inner = self.get_inner();

                    if !inner.#field_ident.unresolved() {
                        let mut_inner = self.get_inner_mut();

                        let result = self.get_transaction().get_repository().find(values).unwrap();

                        mut_inner.#field_ident = yukino::collection::AssociatedEntity::Resolved(result)
                    }

                    inner.get().unwrap()
                }
            };
            let field_setter_token_stream = quote! {
                pub fn #setter_name(&mut self, value: #nested_type) -> &mut Self {
                    let mut_inner = self.get_inner_mut();
                    mut_inner.#field_ident = yukino::collection::AssociatedEntity::Resolved(value.inner());

                    self
                }
            };

            Ok(AchievedFieldResolver {
                field_path: self.field_path.clone(),
                indexes,
                columns: self.columns.clone(),
                joined_table: vec![],
                foreign_keys: vec![ForeignKeyDefinition {
                    name: format!("__{}", self.field_path.1),
                    referenced_table: self.referenced_table.clone().unwrap(),
                    column_map: self.column_map.clone(),
                }],
                data_converter_token_stream,
                converter_getter_ident: converter_getter_name,
                field_getter_ident: getter_name,
                field_getter_token_stream,
                field_setter_ident: setter_name,
                field_setter_token_stream,
                field_type: self.field_type.clone(),
            })
        } else {
            Err(ResolveError::UnexpectedFieldResolverStatus(
                self.field_path.0.clone(),
                self.field_path.1.clone(),
                "WaitingAssemble".to_string(),
                self.status(),
            ))
        }
    }
}
