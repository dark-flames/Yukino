use crate::annotations::FieldAnnotation;
use crate::definitions::{ColumnDefinition, ColumnType, FieldDefinition};
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{ColumnIdent, Expr, JoinClause, Literal, Locatable};
use crate::query::type_check::TypeKind;
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{
    AchievedFieldResolver, EntityName, EntityResolver, FieldPath, FieldResolver, FieldResolverBox,
    FieldResolverSeed, FieldResolverSeedBox, FieldResolverStatus, TypePathResolver, ValueConverter,
};
use crate::types::{
    DatabaseType, DatabaseValue, ExprWrapper, IdentResolveStatus, TypeInfo, TypeResolver, ValuePack,
};
use heck::SnakeCase;
use iroha::ToTokens;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::Type;

pub struct StringFieldResolverSeed;

impl FieldResolverSeed for StringFieldResolverSeed {
    fn new() -> Self
    where
        Self: Sized,
    {
        StringFieldResolverSeed
    }

    fn boxed(&self) -> FieldResolverSeedBox {
        Box::new(StringFieldResolverSeed)
    }

    fn try_breed(
        &self,
        entity_name: EntityName,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
        type_path_resolver: &TypePathResolver,
    ) -> Option<Result<FieldResolverBox, ResolveError>> {
        let (nullable, nested_type) = match Self::unwrap_option(
            field_type,
            (entity_name.clone(), ident.to_string()),
            type_path_resolver,
        ) {
            Ok(r) => r,
            Err(e) => return Some(Err(e)),
        };
        if let Type::Path(type_path) = nested_type {
            if let Some(first_segment) = type_path.path.segments.first() {
                if first_segment.ident == *"String" {
                    let field = Self::default_annotations(annotations);

                    if field.auto_increase {
                        Some(Err(ResolveError::Others(format!(
                            "AutoIncrease is not supported on string field({0} in {1})",
                            ident, entity_name
                        ))))
                    } else {
                        Some(Ok(Box::new(StringFieldResolver {
                            field_path: (entity_name.clone(), ident.to_string()),
                            definition: ColumnDefinition {
                                name: field
                                    .name
                                    .unwrap_or_else(|| ident.to_string().to_snake_case()),
                                ty: ColumnType::NormalColumn(entity_name),
                                data_type: DatabaseType::String,
                                unique: field.unique,
                                auto_increase: field.auto_increase,
                                primary_key: Self::is_primary_key(annotations),
                                nullable,
                            },
                            field_type: type_path_resolver.get_full_type(field_type.clone()),
                            nullable,
                        })))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct StringFieldResolver {
    field_path: FieldPath,
    definition: ColumnDefinition,
    field_type: Type,
    nullable: bool,
}

impl FieldResolver for StringFieldResolver {
    fn status(&self) -> FieldResolverStatus {
        FieldResolverStatus::WaitingAssemble
    }

    fn field_path(&self) -> FieldPath {
        self.field_path.clone()
    }

    fn resolve_by_waiting_entity(
        &mut self,
        _resolver: &EntityResolver,
    ) -> Result<FieldResolverStatus, ResolveError> {
        unreachable!()
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
        let method_name = self.converter_getter_ident();

        let (entity_name, field_name) = self.field_path();

        let converter = StringValueConverter {
            is_primary_key: self.definition.primary_key,
            entity_name,
            field_name: field_name.clone(),
            column_name: self.definition.name.clone(),
        };

        let data_converter_token_stream = quote! {
            pub fn #method_name() -> yukino::resolver::field_resolver_seeds::StringValueConverter {
                #converter
            }
        };

        let getter_name = self.getter_ident();
        let setter_name = self.setter_ident();
        let field_ident = format_ident!("{}", field_name);
        let field_ty = &self.field_type;

        let field_getter_token_stream = quote! {
            pub fn #getter_name(&self) -> &#field_ty{
                let inner = self.get_inner();
                &inner.#field_ident
            }
        };
        let field_setter_token_stream = if self.nullable {
            quote! {
                pub fn #setter_name(&mut self, value: String) -> &mut Self {
                    let inner = self.get_inner_mut();
                    inner.#field_ident= Some(value);
                    self
                }
            }
        } else {
            quote! {
                pub fn #setter_name(&mut self, value: String) -> &mut Self {
                    let inner = self.get_inner_mut();
                    inner.#field_ident= value;
                    self
                }
            }
        };

        Ok(AchievedFieldResolver {
            field_path: self.field_path(),
            indexes: vec![],
            columns: vec![self.definition.clone()],
            joined_table: vec![],
            foreign_keys: vec![],
            data_converter_token_stream,
            converter_getter_ident: method_name,
            field_getter_ident: getter_name,
            field_getter_token_stream,
            field_setter_ident: setter_name,
            field_setter_token_stream,
            field_type: self.field_type.clone(),
            field_definition: FieldDefinition {
                name: self.definition.name.clone(),
                type_resolver_name: StringTypeResolver::seed().name(),
                field_type: "string".to_string(),
                nullable: self.nullable,
                columns: vec![self.definition.name.clone()],
                tables: vec![],
            },
        })
    }
}

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::resolver::field_resolver_seeds")]
pub struct StringValueConverter {
    is_primary_key: bool,
    entity_name: String,
    field_name: String,
    column_name: String,
}

impl ValueConverter<String> for StringValueConverter {
    fn to_field_value(&self, values: &ValuePack) -> Result<String, DataConvertError> {
        match values.get(&self.column_name) {
            Some(DatabaseValue::String(value)) => Ok(value.clone()),
            _ => Err(DataConvertError::UnexpectedDatabaseValueType(
                self.entity_name.clone(),
                self.field_name.clone(),
            )),
        }
    }

    fn to_database_values_by_ref(&self, value: &String) -> Result<ValuePack, DataConvertError> {
        let mut map = HashMap::new();
        map.insert(
            self.column_name.clone(),
            DatabaseValue::String(value.clone()),
        );

        Ok(map)
    }

    fn primary_column_values_by_ref(&self, value: &String) -> Result<ValuePack, DataConvertError> {
        if self.is_primary_key {
            self.to_database_values_by_ref(value)
        } else {
            Ok(HashMap::new())
        }
    }
}

impl ValueConverter<Option<String>> for StringValueConverter {
    fn to_field_value(&self, values: &ValuePack) -> Result<Option<String>, DataConvertError> {
        match values.get(&self.column_name) {
            Some(DatabaseValue::String(value)) => Ok(Some(value.clone())),
            Some(DatabaseValue::Null(DatabaseType::String)) => Ok(None),
            _ => Err(DataConvertError::UnexpectedDatabaseValueType(
                self.entity_name.clone(),
                self.field_name.clone(),
            )),
        }
    }

    fn to_database_values_by_ref(
        &self,
        value: &Option<String>,
    ) -> Result<ValuePack, DataConvertError> {
        let mut map = HashMap::new();
        map.insert(
            self.column_name.clone(),
            match value {
                Some(v) => DatabaseValue::String(v.clone()),
                None => DatabaseValue::Null(DatabaseType::String),
            },
        );

        Ok(map)
    }

    fn primary_column_values_by_ref(
        &self,
        value: &Option<String>,
    ) -> Result<ValuePack, DataConvertError> {
        if self.is_primary_key {
            self.to_database_values_by_ref(value)
        } else {
            Ok(HashMap::new())
        }
    }
}

pub struct StringTypeResolver;

impl TypeResolver for StringTypeResolver {
    fn seed() -> Box<dyn TypeResolver>
    where
        Self: Sized,
    {
        Box::new(StringTypeResolver)
    }

    fn name(&self) -> String {
        "string".to_string()
    }

    fn type_kind(&self) -> TypeKind {
        TypeKind::String
    }

    fn wrap_lit(
        &self,
        lit: &Literal,
        type_info: TypeInfo,
    ) -> Result<(ExprWrapper, Vec<(String, String)>), SyntaxErrorWithPos> {
        if &type_info.field_type != "String" {
            return Err(lit.location().error(SyntaxError::TypeError(
                "String".to_string(),
                type_info.field_type,
            )));
        }

        match lit {
            Literal::String(_) => Ok((
                ExprWrapper {
                    exprs: vec![Expr::Literal(lit.clone())],
                    type_info,
                    location: lit.location(),
                },
                vec![],
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
        if &field_definition.field_type != "String" {
            Err(ident.location().error(SyntaxError::TypeError(
                "String".to_string(),
                field_definition.field_type.clone(),
            )))
        } else {
            let type_info = TypeInfo {
                field_type: field_definition.field_type.clone(),
                nullable: field_definition.nullable,
                resolver_name: self.name(),
                type_kind: self.type_kind(),
            };
            Ok((
                IdentResolveStatus::Resolved(ExprWrapper {
                    exprs: vec![Expr::ColumnIdent(ident.clone())],
                    type_info,
                    location: ident.location(),
                }),
                vec![],
            ))
        }
    }
}
