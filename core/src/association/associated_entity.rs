use crate::annotations::{Association, FieldAnnotation, IndexMethod};
use crate::association::FakeEntity;
use crate::definitions::{
    AssociationDefinition, ColumnDefinition, ColumnType, FieldDefinition, ForeignKeyDefinition,
    IndexDefinition,
};
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    Binary, BinaryOperator, ColumnIdent, Expr, JoinClause, JoinOn, JoinType, Literal, Locatable,
    TableReference,
};
use crate::query::type_check::TypeKind;
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::FieldResolverStatus::WaitingAssemble;
use crate::resolver::{
    AchievedFieldResolver, EntityName, EntityResolver, FieldName, FieldPath, FieldResolver,
    FieldResolverBox, FieldResolverSeed, FieldResolverSeedBox, FieldResolverStatus,
    TypePathResolver, ValueConverter,
};
use crate::types::{
    DatabaseType, DatabaseValue, ExprWrapper, IdentResolveStatus, TypeInfo, TypeResolver, ValuePack,
};
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

#[derive(Clone)]
pub enum AssociatedEntity<E>
where
    E: Entity + Clone,
{
    Unresolved(ValuePack),
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
#[Iroha(mod_path = "yukino::resolver::field_resolver_seeds")]
pub struct AssociatedEntityValueConverter<E: Entity + Clone> {
    entity_name: String,
    field_name: String,
    column_map: HashMap<String, String>,
    column_ty: HashMap<String, DatabaseType>,
    is_primary_key: bool,
    _marker: PhantomData<E>,
}

impl<E: Entity + Clone> ValueConverter<AssociatedEntity<E>> for AssociatedEntityValueConverter<E> {
    fn to_field_value(&self, values: &ValuePack) -> Result<AssociatedEntity<E>, DataConvertError> {
        let value_map: ValuePack = values
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
                        reverse_map
                            .get(&column)
                            .map(|current_column_name| (current_column_name.clone(), value))
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

impl<E: Entity + Clone> ValueConverter<Option<AssociatedEntity<E>>>
    for AssociatedEntityValueConverter<E>
{
    fn to_field_value(
        &self,
        values: &ValuePack,
    ) -> Result<Option<AssociatedEntity<E>>, DataConvertError> {
        let value_map: ValuePack = values
            .iter()
            .filter_map(|(name, value)| {
                if self.column_map.contains_key(name.as_str()) {
                    Some((name.clone(), value.clone()))
                } else {
                    None
                }
            })
            .collect();

        let is_null = value_map
            .values()
            .any(|v| matches!(v, DatabaseValue::Null(_)));
        if is_null {
            Ok(None)
        } else if value_map.len() == self.column_map.len() {
            Ok(Some(AssociatedEntity::Unresolved(value_map)))
        } else {
            Err(DataConvertError::UnexpectedDatabaseValueType(
                self.entity_name.clone(),
                self.field_name.clone(),
            ))
        }
    }

    fn to_database_values_by_ref(
        &self,
        value: &Option<AssociatedEntity<E>>,
    ) -> Result<ValuePack, DataConvertError> {
        match value {
            Some(AssociatedEntity::Unresolved(map)) => Ok(map.clone()),
            Some(AssociatedEntity::Resolved(entity)) => {
                let associated_result = entity.to_database_values()?;

                let reverse_map: HashMap<String, String> = self
                    .column_map
                    .iter()
                    .map(|(column, associated_column)| (associated_column.clone(), column.clone()))
                    .collect();

                Ok(associated_result
                    .into_iter()
                    .filter_map(|(column, value)| {
                        reverse_map
                            .get(&column)
                            .map(|current_column_name| (current_column_name.clone(), value))
                    })
                    .collect())
            }
            None => Ok(self
                .column_ty
                .clone()
                .into_iter()
                .map(|(name, ty)| (name, DatabaseValue::Null(ty)))
                .collect()),
        }
    }

    fn primary_column_values_by_ref(
        &self,
        value: &Option<AssociatedEntity<E>>,
    ) -> Result<ValuePack, DataConvertError> {
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
        let (nullable, option_nested_type) = match Self::unwrap_option(
            field_type,
            (entity_name.clone(), ident.to_string()),
            type_path_resolver,
        ) {
            Ok(r) => r,
            Err(e) => return Some(Err(e)),
        };

        let (old_nested_type, nested_type, mut type_path) = match &option_nested_type {
            Type::Path(type_path) => {
                let full_path = type_path_resolver.get_full_path(type_path.clone());

                let last_segment = full_path.path.segments.last()?;

                if last_segment.ident == "AssociatedEntity" {
                    match &last_segment.arguments {
                        PathArguments::AngleBracketed(arguments) => match arguments.args.first() {
                            Some(GenericArgument::Type(Type::Path(nested_type_path))) => {
                                let mut nested_type_path_new = nested_type_path.clone();
                                let ident = format_ident!(
                                    "{}Inner",
                                    nested_type_path_new.path.segments.first().unwrap().ident
                                );

                                nested_type_path_new
                                    .path
                                    .segments
                                    .first_mut()
                                    .unwrap()
                                    .ident = ident;
                                Some((
                                    nested_type_path.clone(),
                                    nested_type_path_new,
                                    type_path.clone(),
                                ))
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
            .unwrap_or(Association {
                mapped_by: None,
                unique: false,
            });

        let last = type_path.path.segments.last_mut().unwrap();

        if let PathArguments::AngleBracketed(argument) = &mut last.arguments {
            let arg = argument.args.first_mut().unwrap();
            *arg = GenericArgument::Type(Type::Path(nested_type.clone()));
        }

        Some(Ok(Box::new(AssociatedEntityFieldResolver {
            field_path: (entity_name, ident.to_string()),
            field_type: Type::Path(type_path),
            proxy_type: Type::Path(old_nested_type.clone()),
            inner_type: Type::Path(nested_type),
            primary_key: Self::is_primary_key(annotations),
            nullable,
            association,
            status: FieldResolverStatus::WaitingForEntity(
                old_nested_type.to_token_stream().to_string(),
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
    proxy_type: Type,
    inner_type: Type,
    primary_key: bool,
    nullable: bool,
    association: Association,
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
                                    nullable: false,
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

                self.status = WaitingAssemble;

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
            let indexes = if self.association.unique {
                if self.columns.len() == 1 {
                    self.columns.first_mut().unwrap().unique = true;
                    vec![]
                } else {
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
                }
            } else {
                vec![]
            };

            let converter_getter_name =
                quote::format_ident!("get_{}_converter", &self.field_path().1.to_snake_case());

            let convert = AssociatedEntityValueConverter {
                entity_name: self.field_path.0.clone(),
                field_name: self.field_path.1.clone(),
                column_map: self.column_map.iter().cloned().collect(),
                column_ty: self
                    .columns
                    .iter()
                    .map(|definition| (definition.name.clone(), definition.data_type))
                    .collect(),
                is_primary_key: self.primary_key,
                _marker: PhantomData::<FakeEntity>::default(),
            };

            let entity_ident = &self.inner_type;

            let output_type = quote! {
                yukino::resolver::field_resolver_seeds::AssociatedEntityValueConverter::<#entity_ident>
            };

            let data_converter_token_stream = quote! {
                pub fn #converter_getter_name() -> #output_type {
                    #convert
                }
            };

            let getter_name = self.getter_ident();
            let setter_name = self.setter_ident();

            let field_ident = TokenStream::from_str(self.field_path.1.as_str()).unwrap();

            let proxy_type = &self.proxy_type;

            let field_getter_token_stream = if self.nullable {
                quote! {
                    pub fn #getter_name(&self) -> Option<#proxy_type> {
                        use yukino::EntityProxy;
                        let inner = self.get_inner();

                        if let Some(yukino::collection::AssociatedEntity::Unresolved(values)) = &inner.#field_ident {
                            let mut_inner = self.get_inner_mut();

                            let result = self.get_transaction().get_repository().find(values).unwrap();

                            mut_inner.#field_ident = Some(yukino::collection::AssociatedEntity::Resolved(result))
                        }

                        inner.#field_ident.map(
                            |associated_entity| {
                                let entity = associated_entity.get().unwrap().clone();

                                self.get_transaction().create_entity(
                                    move || entity
                                )
                            }
                        )
                    }
                }
            } else {
                quote! {
                    pub fn #getter_name(&self) -> #proxy_type {
                        use yukino::EntityProxy;
                        let inner = self.get_inner();

                        if let yukino::collection::AssociatedEntity::Unresolved(values) = &inner.#field_ident {
                            let mut_inner = self.get_inner_mut();

                            let result = self.get_transaction().get_repository().find(values).unwrap();

                            mut_inner.#field_ident = yukino::collection::AssociatedEntity::Resolved(result)
                        }

                        let entity = inner.#field_ident.get().unwrap().clone();

                        self.get_transaction().create_entity(
                            move || entity
                        )
                    }
                }
            };
            let field_setter_token_stream = if self.nullable {
                quote! {
                    pub fn #setter_name(&mut self, value: #proxy_type) -> &mut Self {
                        use yukino::EntityProxy;
                        let mut_inner = self.get_inner_mut();
                        mut_inner.#field_ident = Some(yukino::collection::AssociatedEntity::Resolved(value.inner()));

                        self
                    }
                }
            } else {
                quote! {
                    pub fn #setter_name(&mut self, value: #proxy_type) -> &mut Self {
                        use yukino::EntityProxy;
                        let mut_inner = self.get_inner_mut();
                        mut_inner.#field_ident = yukino::collection::AssociatedEntity::Resolved(value.inner());

                        self
                    }
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
                field_definition: FieldDefinition {
                    entity: self.field_path.1.clone(),
                    name: self.field_path.1.clone(),
                    type_resolver_name: "".to_string(), //todo: impl association type resolver
                    field_type: self.proxy_type.to_token_stream().to_string(),
                    nullable: self.nullable,
                    columns: self
                        .columns
                        .iter()
                        .map(|column_definition| column_definition.name.clone())
                        .collect(),
                    tables: vec![],
                    association: Some(AssociationDefinition {
                        referenced_entity: self.proxy_type.to_token_stream().to_string(),
                        is_list: false,
                        column_map: self.column_map.clone(),
                    }),
                },
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

pub struct AssociatedEntityTypeResolver;

impl TypeResolver for AssociatedEntityTypeResolver {
    fn seed() -> Box<dyn TypeResolver>
    where
        Self: Sized,
    {
        Box::new(AssociatedEntityTypeResolver)
    }

    fn name(&self) -> String {
        "associated_object".to_string()
    }

    fn type_kind(&self) -> TypeKind {
        TypeKind::Others("AssociatedEntity".to_string())
    }

    fn wrap_lit(
        &self,
        lit: &Literal,
        type_info: TypeInfo,
    ) -> Result<(ExprWrapper, Vec<(String, String)>), SyntaxErrorWithPos> {
        match lit {
            Literal::External(external) => Ok((
                ExprWrapper {
                    exprs: vec![Expr::Literal(lit.clone())],
                    type_info: type_info.clone(),
                    location: lit.location(),
                },
                vec![(external.ident.clone(), type_info.field_type)],
            )),
            Literal::Null(_) if type_info.nullable => Ok((
                ExprWrapper {
                    exprs: vec![Expr::Literal(lit.clone())],
                    type_info,
                    location: lit.location(),
                },
                vec![],
            )),
            _ => Err(lit.location().error(SyntaxError::TypeError(
                type_info.to_string(),
                TypeKind::from(lit).to_string(),
            ))),
        }
    }

    fn wrap_ident(
        &self,
        ident: &ColumnIdent,
        field_definition: &FieldDefinition,
    ) -> Result<(IdentResolveStatus, Vec<JoinClause>), SyntaxErrorWithPos> {
        let association = field_definition.association.as_ref().unwrap();
        let location = ident.location();
        let self_alias = ident.segments.first().unwrap();
        let ref_alias = format!(
            "__{}_{}_{}",
            &field_definition.entity, &field_definition.name, &association.referenced_entity
        );
        let mut exprs: Vec<_> = association
            .column_map
            .iter()
            .map(|(left_field, right_field)| {
                Expr::Binary(Binary {
                    operator: BinaryOperator::Eq,
                    left: Box::new(Expr::ColumnIdent(ColumnIdent {
                        segments: vec![self_alias.clone(), left_field.clone()],
                        location,
                    })),
                    right: Box::new(Expr::ColumnIdent(ColumnIdent {
                        segments: vec![ref_alias.clone(), right_field.clone()],
                        location,
                    })),
                    location,
                })
            })
            .collect();

        let mut on = exprs.pop().unwrap();

        for expr in exprs {
            on = Expr::Binary(Binary {
                operator: BinaryOperator::And,
                left: Box::new(on),
                right: Box::new(expr),
                location,
            })
        }

        let join = JoinClause::JoinOn(JoinOn {
            ty: JoinType::Inner,
            table: TableReference {
                name: association.referenced_entity.clone(),
                alias: Some(ref_alias.clone()),
                location,
            },
            on,
            location,
        });

        let mut segments = ident.segments.clone();

        segments.pop();
        segments[0] = ref_alias;

        if segments.len() == 1 {
            segments.push("*".to_string());
        };

        if segments.len() > 2 {
            Ok((
                IdentResolveStatus::Unresolved(ColumnIdent { segments, location }),
                vec![join],
            ))
        } else {
            Ok((
                IdentResolveStatus::Resolved(ExprWrapper {
                    exprs: vec![Expr::ColumnIdent(ColumnIdent { segments, location })],
                    type_info: TypeInfo {
                        resolver_name: self.name(),
                        field_type: field_definition.field_type.clone(),
                        nullable: field_definition.nullable,
                        type_kind: self.type_kind(),
                    },
                    location,
                }),
                vec![join],
            ))
        }
    }
}
