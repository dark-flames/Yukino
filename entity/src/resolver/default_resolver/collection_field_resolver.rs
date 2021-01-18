use crate::annotations::FieldAnnotation;
use crate::definitions::{ColumnDefinition, ColumnType};
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{
    AchievedFieldResolver, ConstructableFieldResolverSeed, EntityPath, EntityResolver, FieldName,
    FieldPath, FieldResolver, FieldResolverBox, FieldResolverSeed, FieldResolverStatus,
};
use crate::types::DatabaseType;
use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{PathSegment, Type};

enum CollectionType {
    List,
    Map,
}

#[allow(dead_code)]
impl CollectionType {
    pub fn from_last_segment(segment: &PathSegment) -> Option<Self> {
        let ident_string = segment.ident.to_string();

        match ident_string.as_str() {
            "Vec" => Some(CollectionType::List),
            "HashMap" => Some(CollectionType::Map),
            _ => None,
        }
    }

    // pub fn converter_token_stream(&self, column_name: String) -> TokenStream {
    // match self {
    // CollectionType::List => (ListValueConverter { column_name }).to_token_stream(),
    // CollectionType::Map => (MapValueConverter { column_name }).to_token_stream(),
    // }
    // }

    pub fn get_converter_name(&self) -> TokenStream {
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

impl ConstructableFieldResolverSeed for CollectionFieldResolverSeed {
    fn new() -> Self
    where
        Self: Sized,
    {
        CollectionFieldResolverSeed
    }
}

#[allow(dead_code)]
impl FieldResolverSeed for CollectionFieldResolverSeed {
    fn try_breed(
        &self,
        _entity_path: EntityPath,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
    ) -> Result<FieldResolverBox, ResolveError> {
        let ty = match field_type {
            Type::Path(type_path) => match type_path.path.segments.iter().rev().next() {
                Some(last_segment) => CollectionType::from_last_segment(&last_segment),
                None => None,
            },
            _ => None,
        }
        .ok_or_else(|| {
            DataConvertError::UnsupportedFieldType(
                field_type.to_token_stream().to_string(),
                "NumericFieldResolverSeed",
            )
        })?;
        let field = Self::default_annotations(annotations);

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

        Ok(Box::new(CollectionFieldResolver {
            _field_path: ("".to_string(), "".to_string()),
            _ty: ty,
            _definition: definition,
        }))
    }
}

pub struct CollectionFieldResolver {
    _field_path: FieldPath,
    _ty: CollectionType,
    _definition: ColumnDefinition,
}

#[allow(dead_code)]
impl FieldResolver for CollectionFieldResolver {
    fn status(&self) -> FieldResolverStatus {
        unimplemented!()
    }

    fn field_path(&self) -> (EntityPath, FieldName) {
        unimplemented!()
    }

    fn entity_path(&self) -> EntityPath {
        unimplemented!()
    }

    fn resolve_by_waiting_entity(
        &mut self,
        _resolver: &EntityResolver,
    ) -> Result<FieldResolverStatus, ResolveError> {
        unimplemented!()
    }

    fn resolve_by_waiting_fields(
        &mut self,
        _resolvers: Vec<&AchievedFieldResolver>,
    ) -> Result<FieldResolverStatus, ResolveError> {
        unimplemented!()
    }

    fn assemble(
        &mut self,
        _entity_resolver: &EntityResolver,
    ) -> Result<AchievedFieldResolver, ResolveError> {
        unimplemented!()
    }
}
