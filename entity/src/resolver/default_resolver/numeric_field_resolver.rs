use crate::annotations::{Field, FieldAnnotation};
use crate::definitions::{ColumnDefinition, ColumnType};
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{
    AchievedFieldResolver, ConstructableFieldResolverSeed, EntityPath, EntityResolver, FieldPath,
    FieldResolver, FieldResolverBox, FieldResolverSeed, FieldResolverStatus, ValueConverter,
};
use crate::types::{DatabaseType, DatabaseValue};
use heck::SnakeCase;
use iroha::ToTokens;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use syn::Type;

enum NumericType {
    Integer(usize),
    UnsignedInteger(usize),
    Float(usize),
}

impl NumericType {
    pub fn from_ident(ident: &Ident) -> Option<Self> {
        let ident_string = ident.to_string();

        match ident_string.as_str() {
            "i16" => Some(NumericType::Integer(16)),
            "i32" => Some(NumericType::Integer(32)),
            "i64" => Some(NumericType::Integer(64)),
            "u16" => Some(NumericType::UnsignedInteger(16)),
            "u32" => Some(NumericType::UnsignedInteger(32)),
            "u64" => Some(NumericType::UnsignedInteger(64)),
            "f32" => Some(NumericType::Float(32)),
            "f64" => Some(NumericType::Float(64)),
            _ => None,
        }
    }

    pub fn converter_token_stream(
        &self,
        column_name: String,
        field_path: FieldPath,
    ) -> TokenStream {
        let field_name = field_path.1.clone();
        let entity_path = field_path.0;
        match self {
            NumericType::Integer(16) => (SmallIntegerValueConverter {
                column_name,
                field_name,
                entity_path,
            })
            .to_token_stream(),
            NumericType::Integer(32) => (IntegerValueConverter {
                column_name,
                field_name,
                entity_path,
            })
            .to_token_stream(),
            NumericType::Integer(64) => (BigIntegerValueConverter {
                column_name,
                field_name,
                entity_path,
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(16) => (UnsignedSmallIntegerValueConverter {
                column_name,
                field_name,
                entity_path,
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(32) => (UnsignedIntegerValueConverter {
                column_name,
                field_name,
                entity_path,
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(64) => (UnsignedBigIntegerValueConverter {
                column_name,
                field_name,
                entity_path,
            })
            .to_token_stream(),
            NumericType::Float(32) => (FloatValueConverter {
                column_name,
                field_name,
                entity_path,
            })
            .to_token_stream(),
            NumericType::Float(64) => (DoubleValueConverter {
                column_name,
                field_name,
                entity_path,
            })
            .to_token_stream(),
            _ => unreachable!(),
        }
    }

    pub fn converter_name(&self) -> TokenStream {
        let prefix = quote::quote! {
            yukino::resolver::default_resolver
        };

        match self {
            NumericType::Integer(16) => quote::quote! {
                #prefix::SmallIntegerValueConverter
            },
            NumericType::Integer(32) => quote::quote! {
                #prefix::IntegerValueConverter
            },
            NumericType::Integer(64) => quote::quote! {
                #prefix::BigIntegerValueConverter
            },
            NumericType::UnsignedInteger(16) => quote::quote! {
                #prefix::UnsignedSmallIntegerValueConverter
            },
            NumericType::UnsignedInteger(32) => quote::quote! {
                #prefix::UnsignedIntegerValueConverter
            },
            NumericType::UnsignedInteger(64) => quote::quote! {
                #prefix::UnsignedBigIntegerValueConverter
            },
            NumericType::Float(32) => quote::quote! {
                #prefix::FloatValueConverter
            },
            NumericType::Float(64) => quote::quote! {
                #prefix::DoubleValueConverter
            },
            _ => unreachable!(),
        }
    }

    pub fn database_type(&self) -> DatabaseType {
        match self {
            NumericType::Integer(16) => DatabaseType::SmallInteger,
            NumericType::Integer(32) => DatabaseType::Integer,
            NumericType::Integer(64) => DatabaseType::BigInteger,
            NumericType::UnsignedInteger(16) => DatabaseType::UnsignedSmallInteger,
            NumericType::UnsignedInteger(32) => DatabaseType::UnsignedInteger,
            NumericType::UnsignedInteger(64) => DatabaseType::UnsignedBigInteger,
            NumericType::Float(32) => DatabaseType::Float,
            NumericType::Float(64) => DatabaseType::Double,
            _ => unreachable!(),
        }
    }
}

pub struct NumericFieldResolverSeed;

impl ConstructableFieldResolverSeed for NumericFieldResolverSeed {
    fn new() -> Self
    where
        Self: Sized,
    {
        NumericFieldResolverSeed
    }
}

impl FieldResolverSeed for NumericFieldResolverSeed {
    fn try_breed(
        &self,
        entity_path: EntityPath,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
    ) -> Result<FieldResolverBox, ResolveError> {
        let ty = match field_type {
            Type::Path(type_path) => match type_path.path.segments.first() {
                Some(first_segment) => NumericType::from_ident(&first_segment.ident),
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

        let primary_key = annotations
            .iter()
            .any(|attr| matches!(attr, FieldAnnotation::ID(_)));

        let default_annotation = Field {
            name: None,
            unique: false,
            auto_increase: false,
            options: None,
        };

        let field = annotations
            .iter()
            .filter_map(|attr| match attr {
                FieldAnnotation::Field(field_annotation) => Some(field_annotation),
                _ => None,
            })
            .next()
            .unwrap_or(&default_annotation);

        let definition = ColumnDefinition {
            name: field
                .name
                .as_ref()
                .cloned()
                .unwrap_or_else(|| ident.to_string().to_snake_case()),
            ty: ColumnType::NormalColumn(ident.to_string()),
            data_type: ty.database_type(),
            unique: field.unique,
            auto_increase: field.auto_increase,
            primary_key,
        };

        Ok(Box::new(NumericFieldResolver {
            field_path: (entity_path, ident.to_string()),
            ty,
            definition,
        }))
    }
}

pub struct NumericFieldResolver {
    field_path: FieldPath,
    ty: NumericType,
    definition: ColumnDefinition,
}

impl FieldResolver for NumericFieldResolver {
    fn status(&self) -> FieldResolverStatus {
        FieldResolverStatus::WaitingAssemble
    }

    fn field_path(&self) -> FieldPath {
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
        let method_name = quote::format_ident!("get_{}_converter", &self.field_path().1);
        let output_type = self.ty.converter_name();
        let converter = self
            .ty
            .converter_token_stream(self.definition.name.clone(), self.field_path.clone());

        let data_converter_token_stream = quote::quote! {
            pub fn #method_name() -> #output_type {
                #converter
            }
        };

        Ok(AchievedFieldResolver {
            field_path: self.field_path.clone(),
            columns: vec![self.definition.clone()],
            joined_table: vec![],
            foreign_keys: vec![],
            data_converter_token_stream,
            converter_getter_ident: method_name,
        })
    }
}

macro_rules! impl_converter {
    ($ident: ident, $output_type: ty, $database_value: ident) => {
        #[derive(ToTokens)]
        #[Iroha(mod_path = "yukino::resolver::default_resolver")]
        pub struct $ident {
            column_name: String,
            entity_path: String,
            field_name: String,
        }

        impl ValueConverter<$output_type> for $ident {
            fn to_value(
                &self,
                values: &HashMap<String, DatabaseValue>,
            ) -> Result<$output_type, DataConvertError> {
                match values.get(&self.column_name) {
                    Some(DatabaseValue::$database_value(value)) => Ok(*value),
                    _ => Err(DataConvertError::UnexpectedDatabaseValue(
                        self.entity_path.clone(),
                        self.field_name.clone(),
                    )),
                }
            }

            fn to_database_value_by_ref(
                &self,
                value: &$output_type,
            ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    self.column_name.clone(),
                    DatabaseValue::$database_value(*value),
                );

                Ok(map)
            }
        }
    };
}

impl_converter!(SmallIntegerValueConverter, i16, SmallInteger);
impl_converter!(IntegerValueConverter, i32, Integer);
impl_converter!(BigIntegerValueConverter, i64, BigInteger);
impl_converter!(
    UnsignedSmallIntegerValueConverter,
    u16,
    UnsignedSmallInteger
);
impl_converter!(UnsignedIntegerValueConverter, u32, UnsignedInteger);
impl_converter!(UnsignedBigIntegerValueConverter, u64, UnsignedBigInteger);
impl_converter!(FloatValueConverter, f32, Float);
impl_converter!(DoubleValueConverter, f64, Double);
