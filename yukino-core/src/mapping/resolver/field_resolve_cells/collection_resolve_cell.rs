use crate::mapping::definition::{ColumnDefinition, ForeignKeyDefinition, TableDefinition};
use crate::mapping::resolver::entity_resolve_cell::EntityResolveCell;
use crate::mapping::resolver::error::{ResolveError, UnresolvedError};
use crate::mapping::resolver::{
    ConstructableCell, FieldPath, FieldResolveCell, FieldResolveStatus, ValueConverter,
};
use crate::mapping::{Column, DatabaseType, DatabaseValue, FieldAttribute};
use crate::ParseError;
use iroha::ToTokens;
use proc_macro2::{Ident, TokenStream};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::{from_value, to_value};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::Hash;
use syn::export::ToTokens;
use syn::{PathSegment, Type};

enum CollectionType {
    /// Array with nested type tokens
    Array,
    /// Map with key type and value type tokens
    Map,
}

impl CollectionType {
    pub fn from_last_segment(segment: &PathSegment) -> Option<Self> {
        let ident_string = segment.ident.to_string();

        match ident_string.as_str() {
            "Vec" => Some(CollectionType::Array),
            "HashMap" => Some(CollectionType::Map),
            _ => None,
        }
    }

    pub fn converter_token_stream(&self, column_name: String) -> TokenStream {
        match self {
            CollectionType::Array => (ArrayValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            CollectionType::Map => (MapValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
        }
    }

    pub fn get_converter_name(&self) -> TokenStream {
        match self {
            CollectionType::Array => quote::quote! {
                yukino::mapping::resolver::ArrayValueConverter
            },
            CollectionType::Map => quote::quote! {
                yukino::mapping::resolver::MapValueConverter
            },
        }
    }
}

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct ArrayValueConverter {
    column_name: String,
}

impl<T> ValueConverter<Vec<T>> for ArrayValueConverter
where
    T: Serialize + DeserializeOwned,
{
    fn to_value(&self, values: &HashMap<String, DatabaseValue>) -> Result<Vec<T>, ParseError> {
        match values.get(&self.column_name) {
            Some(DatabaseValue::Json(value)) if value.is_array() => Ok(from_value(value.clone())
                .map_err(|e| {
                let message = format!(
                    "Some Error occur while parsing field {}: {}",
                    &self.column_name,
                    e.to_string()
                );

                ParseError::new(message.as_str())
            })?),
            _ => {
                let message = format!("Unexpected DatabaseValue on field {}", &self.column_name);
                Err(ParseError::new(&message))
            }
        }
    }

    fn to_database_value(
        &self,
        value: &Vec<T>,
    ) -> Result<HashMap<String, DatabaseValue>, ParseError> {
        let json_value = to_value(value).map_err(|e| {
            let message = format!(
                "Some Error occur while serializing field {}: {}",
                &self.column_name,
                e.to_string()
            );

            ParseError::new(message.as_str())
        })?;

        let key = self.column_name.clone();

        let mut result = HashMap::new();

        result.insert(key, DatabaseValue::Json(json_value));

        Ok(result)
    }
}

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct MapValueConverter {
    column_name: String,
}

impl<K, V> ValueConverter<HashMap<K, V>> for MapValueConverter
where
    K: Eq + Hash + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    fn to_value(
        &self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<HashMap<K, V>, ParseError> {
        match values.get(&self.column_name) {
            Some(DatabaseValue::Json(value)) if value.is_object() => Ok(from_value(value.clone())
                .map_err(|e| {
                    let message = format!(
                        "Some Error occur while parsing field {}: {}",
                        &self.column_name,
                        e.to_string()
                    );

                    ParseError::new(message.as_str())
                })?),
            _ => {
                let message = format!("Unexpected DatabaseValue on field {}", &self.column_name);
                Err(ParseError::new(&message))
            }
        }
    }

    fn to_database_value(
        &self,
        value: &HashMap<K, V>,
    ) -> Result<HashMap<String, DatabaseValue>, ParseError> {
        let json_value = to_value(value).map_err(|e| {
            let message = format!(
                "Some Error occur while serializing field {}: {}",
                &self.column_name,
                e.to_string()
            );

            ParseError::new(message.as_str())
        })?;

        let key = self.column_name.clone();

        let mut result = HashMap::new();

        result.insert(key, DatabaseValue::Json(json_value));

        Ok(result)
    }
}

pub struct CollectionResolveCell {
    status: FieldResolveStatus,
    ty: Option<CollectionType>,
    entity_name: Option<String>,
    field_ident: Option<Ident>,
    column: Option<Column>,
}

impl ConstructableCell for CollectionResolveCell {
    fn get_seed() -> Self
    where
        Self: Sized,
    {
        CollectionResolveCell {
            status: FieldResolveStatus::Seed,
            ty: None,
            entity_name: None,
            field_ident: None,
            column: None,
        }
    }
}

impl FieldResolveCell for CollectionResolveCell {
    fn weight(&self) -> usize {
        1
    }

    fn get_status(&self) -> FieldResolveStatus {
        self.status.clone()
    }

    fn resolve_fields(
        &mut self,
        _fields: HashMap<FieldPath, &dyn FieldResolveCell, RandomState>,
    ) -> Result<FieldResolveStatus, ResolveError> {
        unreachable!()
    }

    fn resolve_entity(
        &mut self,
        _entity: &EntityResolveCell,
    ) -> Result<FieldResolveStatus, ResolveError> {
        unreachable!()
    }

    fn assemble(
        &mut self,
        _entity: &EntityResolveCell,
    ) -> Result<FieldResolveStatus, ResolveError> {
        unreachable!()
    }

    fn field_name(&self) -> Result<String, UnresolvedError> {
        self.field_ident
            .as_ref()
            .map(|name| name.to_string())
            .ok_or_else(|| UnresolvedError::new("Collection Resolve cell"))
    }

    fn column_names(&self) -> Result<Vec<String>, UnresolvedError> {
        self.field_name().map(|field_name| {
            vec![self
                .column
                .as_ref()
                .map(|column| column.name.clone().unwrap_or_else(|| field_name.clone()))
                .unwrap_or(field_name)]
        })
    }

    fn entity_name(&self) -> Result<String, UnresolvedError> {
        self.entity_name
            .as_ref()
            .cloned()
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))
    }

    fn is_primary_key(&self) -> Result<bool, UnresolvedError> {
        if self.status.is_finished() {
            Ok(false)
        } else {
            Err(UnresolvedError::new("Integer Resolve cell"))
        }
    }

    fn get_foreigner_keys(&self) -> Result<Vec<ForeignKeyDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Finished => Ok(vec![]),
            _ => Err(UnresolvedError::new("Collection Resolve cell")),
        }
    }

    fn get_column_definitions(&self) -> Result<Vec<ColumnDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Finished => Ok(vec![ColumnDefinition {
                name: self.column_names()?[0].clone(),
                column_type: DatabaseType::Json,
                unique: false,
                auto_increase: false,
                is_primary_key: false,
            }]),
            _ => Err(UnresolvedError::new("Integer Resolve cell")),
        }
    }

    fn get_joined_table_definitions(&self) -> Result<Vec<TableDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Finished => Ok(vec![]),
            _ => Err(UnresolvedError::new("Collection Resolve cell")),
        }
    }

    fn get_data_converter_token_stream(&self) -> Result<TokenStream, UnresolvedError> {
        let column_name = self.column_names()?[0].clone();
        let converter = self
            .ty
            .as_ref()
            .ok_or_else(|| UnresolvedError::new("Collection resolve cell"))?
            .converter_token_stream(column_name);

        let method_name = self.get_data_converter_getter_ident()?;
        let output_type = self.ty.as_ref().unwrap().get_converter_name();

        Ok(quote::quote! {
            pub fn #method_name() -> #output_type {
                #converter
            }
        })
    }

    fn get_data_converter_getter_ident(&self) -> Result<Ident, UnresolvedError> {
        Ok(quote::format_ident!("get_{}_converter", self.field_name()?))
    }

    fn breed(
        &self,
        entity_name: String,
        ident: &Ident,
        attributes: &[FieldAttribute],
        field_type: &Type,
    ) -> Result<Box<dyn FieldResolveCell>, ResolveError> {
        let ty = match field_type {
            Type::Path(type_path) => match type_path.path.segments.iter().rev().next() {
                Some(last_segment) => CollectionType::from_last_segment(&last_segment),
                None => None,
            },
            _ => None,
        }
        .ok_or_else(|| {
            let message = format!(
                "{} is not supported by collection resolve cell",
                field_type.to_token_stream().to_string()
            );
            ResolveError::new(&entity_name, &message)
        })?;

        let column = attributes
            .iter()
            .filter_map(|attr| match attr {
                FieldAttribute::Column(column) => Some(column.clone()),
                _ => None,
            })
            .next();

        Ok(Box::new(CollectionResolveCell {
            status: FieldResolveStatus::Finished,
            ty: Some(ty),
            entity_name: Some(entity_name),
            field_ident: Some(ident.clone()),
            column,
        }))
    }

    fn match_field(&self, _attributes: &[FieldAttribute], field_type: &Type) -> bool {
        match field_type {
            Type::Path(type_path) => match type_path.path.segments.iter().rev().next() {
                Some(last_segment) => CollectionType::from_last_segment(&last_segment).is_some(),
                None => false,
            },
            _ => false,
        }
    }
}
