use crate::annotations::FieldAnnotation;
use crate::definitions::{ColumnDefinition, ColumnType};
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{AchievedFieldResolver, EntityName, EntityResolver, FieldPath, FieldResolver, FieldResolverBox, FieldResolverSeed, FieldResolverStatus, ValueConverter, TypePathResolver, FieldResolverSeedBox};
use crate::types::{DatabaseType, DatabaseValue};
use heck::SnakeCase;
use iroha::ToTokens;
use proc_macro2::Ident;
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
        type_path_resolver: &TypePathResolver
    ) -> Option<Result<FieldResolverBox, ResolveError>> {
        if let Type::Path(type_path) = field_type {
            if let Some(last_segment) = type_path_resolver.get_full_path(type_path).iter().rev().next() {
                if last_segment == "String" {
                    let field = Self::default_annotations(annotations);
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
                        },
                    })))
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
        let method_name = self.default_converter_getter_ident();

        let (entity_name, field_name) = self.field_path();

        let converter = StringValueConverter {
            is_primary_key: self.definition.primary_key,
            entity_name,
            field_name,
            column_name: self.definition.name.clone(),
        };

        let data_converter_token_stream = quote::quote! {
            pub fn #method_name() -> yukino::resolver::default_resolver::StringValueConverter {
                #converter
            }
        };

        Ok(AchievedFieldResolver {
            field_path: self.field_path(),
            columns: vec![self.definition.clone()],
            joined_table: vec![],
            foreign_keys: vec![],
            data_converter_token_stream,
            converter_getter_ident: method_name,
        })
    }
}

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::resolver::default_resolver")]
pub struct StringValueConverter {
    is_primary_key: bool,
    entity_name: String,
    field_name: String,
    column_name: String,
}

impl ValueConverter<String> for StringValueConverter {
    fn to_field_value(
        &self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<String, DataConvertError> {
        match values.get(&self.column_name) {
            Some(DatabaseValue::String(value)) => Ok(value.clone()),
            _ => Err(DataConvertError::UnexpectedDatabaseValueType(
                self.entity_name.clone(),
                self.field_name.clone(),
            )),
        }
    }

    fn to_database_values_by_ref(
        &self,
        value: &String,
    ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
        let mut map = HashMap::new();
        map.insert(
            self.column_name.clone(),
            DatabaseValue::String(value.clone()),
        );

        Ok(map)
    }

    fn primary_column_values_by_ref(
        &self,
        value: &String,
    ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
        if self.is_primary_key {
            self.to_database_values_by_ref(value)
        } else {
            Ok(HashMap::new())
        }
    }
}
