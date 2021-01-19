use crate::annotations::FieldAnnotation;
use crate::definitions::{ColumnDefinition, ColumnType};
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{
    AchievedFieldResolver, EntityPath, EntityResolver, FieldName, FieldPath, FieldResolver,
    FieldResolverBox, FieldResolverSeed, FieldResolverStatus, ValueConverter,
};
use crate::types::{DatabaseType, DatabaseValue};
use heck::SnakeCase;
use iroha::ToTokens;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_value, to_value};
use std::collections::HashMap;
use std::hash::Hash;
use syn::{PathSegment, Type};

enum CollectionType {
    List,
    Map,
}

impl CollectionType {
    pub fn from_last_segment(segment: &PathSegment) -> Option<Self> {
        let ident_string = segment.ident.to_string();

        match ident_string.as_str() {
            "Vec" => Some(CollectionType::List),
            "HashMap" => Some(CollectionType::Map),
            _ => None,
        }
    }

    pub fn converter_token_stream(
        &self,
        column_name: String,
        field_path: FieldPath,
    ) -> TokenStream {
        match self {
            CollectionType::List => (ListValueConverter {
                entity_path: field_path.0.clone(),
                field_name: field_path.1,
                column_name,
            })
            .to_token_stream(),
            CollectionType::Map => (MapValueConverter {
                entity_path: field_path.0.clone(),
                field_name: field_path.1,
                column_name,
            })
            .to_token_stream(),
        }
    }

    pub fn converter_name(&self) -> TokenStream {
        match self {
            CollectionType::List => quote::quote! {
                yukino::resolver::default_resolver::ListValueConverter
            },
            CollectionType::Map => quote::quote! {
                yukino::resolver::default_resolver::MapValueConverter
            },
        }
    }
}

pub struct CollectionFieldResolverSeed;

impl FieldResolverSeed for CollectionFieldResolverSeed {
    fn new() -> Self
    where
        Self: Sized,
    {
        CollectionFieldResolverSeed
    }
    fn try_breed(
        &self,
        entity_path: EntityPath,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
    ) -> Option<Result<FieldResolverBox, ResolveError>> {
        let ty = match field_type {
            Type::Path(type_path) => match type_path.path.segments.iter().rev().next() {
                Some(last_segment) => CollectionType::from_last_segment(&last_segment),
                None => None,
            },
            _ => None,
        }?;

        let field = Self::default_annotations(annotations);
        if field.unique || field.auto_increase || Self::is_primary_key(annotations) {
            Some(Err(ResolveError::Others(
                format!(
                    "PrimaryKey Unique or AutoIncrease is not supported on collection field({0} in {1})",
                    ident,
                    entity_path
                )
            )))
        } else {
            let definition = ColumnDefinition {
                name: field
                    .name
                    .unwrap_or_else(|| ident.to_string().to_snake_case()),
                ty: ColumnType::NormalColumn(ident.to_string()),
                data_type: DatabaseType::Json,
                unique: false,
                auto_increase: false,
                primary_key: false,
            };

            Some(Ok(Box::new(CollectionFieldResolver {
                field_path: (entity_path, ident.to_string()),
                ty,
                definition,
            })))
        }
    }
}

pub struct CollectionFieldResolver {
    field_path: FieldPath,
    ty: CollectionType,
    definition: ColumnDefinition,
}

impl FieldResolver for CollectionFieldResolver {
    fn status(&self) -> FieldResolverStatus {
        FieldResolverStatus::WaitingAssemble
    }

    fn field_path(&self) -> (EntityPath, FieldName) {
        self.field_path.clone()
    }

    fn entity_path(&self) -> EntityPath {
        self.field_path.0.clone()
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
        let output_type = self.ty.converter_name();
        let converter = self
            .ty
            .converter_token_stream(self.definition.name.clone(), self.field_path());

        let data_converter_token_stream = quote::quote! {
            pub fn #method_name() -> #output_type {
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
pub struct ListValueConverter {
    entity_path: String,
    field_name: String,
    column_name: String,
}

impl<T> ValueConverter<Vec<T>> for ListValueConverter
where
    T: Serialize + DeserializeOwned,
{
    fn to_field_value(
        &self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<Vec<T>, DataConvertError> {
        match values.get(&self.column_name) {
            Some(DatabaseValue::Json(value)) if value.is_array() => from_value(value.clone())
                .map_err(|e| {
                    DataConvertError::DatabaseValueConvertError(
                        e.to_string(),
                        self.entity_path.clone(),
                        self.field_name.clone(),
                    )
                }),
            _ => Err(DataConvertError::UnexpectedDatabaseValueType(
                self.entity_path.clone(),
                self.field_name.clone(),
            )),
        }
    }

    fn to_database_values_by_ref(
        &self,
        value: &Vec<T>,
    ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
        let json_value = to_value(value).map_err(|e| {
            DataConvertError::DatabaseValueConvertError(
                e.to_string(),
                self.entity_path.clone(),
                self.field_name.clone(),
            )
        })?;

        let key = self.column_name.clone();

        let mut result = HashMap::new();

        result.insert(key, DatabaseValue::Json(json_value));

        Ok(result)
    }

    fn primary_column_values_by_ref(
        &self,
        _value: &Vec<T>,
    ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
        Ok(HashMap::new())
    }
}

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::resolver::default_resolver")]
pub struct MapValueConverter {
    entity_path: String,
    field_name: String,
    column_name: String,
}

impl<K, V> ValueConverter<HashMap<K, V>> for MapValueConverter
where
    K: Eq + Hash + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    fn to_field_value(
        &self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<HashMap<K, V>, DataConvertError> {
        match values.get(&self.column_name) {
            Some(DatabaseValue::Json(value)) if value.is_object() => from_value(value.clone())
                .map_err(|e| {
                    DataConvertError::DatabaseValueConvertError(
                        e.to_string(),
                        self.entity_path.clone(),
                        self.field_name.clone(),
                    )
                }),
            _ => Err(DataConvertError::UnexpectedDatabaseValueType(
                self.entity_path.clone(),
                self.field_name.clone(),
            )),
        }
    }

    fn to_database_values_by_ref(
        &self,
        value: &HashMap<K, V>,
    ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
        let json_value = to_value(value).map_err(|e| {
            DataConvertError::DatabaseValueConvertError(
                e.to_string(),
                self.entity_path.clone(),
                self.field_name.clone(),
            )
        })?;

        let key = self.column_name.clone();

        let mut result = HashMap::new();

        result.insert(key, DatabaseValue::Json(json_value));

        Ok(result)
    }

    fn primary_column_values_by_ref(
        &self,
        _value: &HashMap<K, V>,
    ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
        Ok(HashMap::new())
    }
}
