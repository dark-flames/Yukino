use crate::annotations::FieldAnnotation;
use crate::definitions::{ColumnDefinition, ColumnType};
use crate::resolver::error::{DataConvertError, ResolveError};
use crate::resolver::{
    AchievedFieldResolver, EntityName, EntityResolver, FieldPath, FieldResolver, FieldResolverBox,
    FieldResolverSeed, FieldResolverSeedBox, FieldResolverStatus, TypePathResolver, ValueConverter,
};
use crate::types::{DatabaseType, DatabaseValue};
use heck::SnakeCase;
use iroha::ToTokens;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{Type, TypePath};

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
        is_primary_key: bool,
    ) -> TokenStream {
        let field_name = field_path.1.clone();
        let entity_name = field_path.0;
        match self {
            NumericType::Integer(16) => (SmallIntegerValueConverter {
                is_primary_key,
                column_name,
                field_name,
                entity_name,
            })
            .to_token_stream(),
            NumericType::Integer(32) => (IntegerValueConverter {
                is_primary_key,
                column_name,
                field_name,
                entity_name,
            })
            .to_token_stream(),
            NumericType::Integer(64) => (BigIntegerValueConverter {
                is_primary_key,
                column_name,
                field_name,
                entity_name,
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(16) => (UnsignedSmallIntegerValueConverter {
                is_primary_key,
                column_name,
                field_name,
                entity_name,
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(32) => (UnsignedIntegerValueConverter {
                is_primary_key,
                column_name,
                field_name,
                entity_name,
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(64) => (UnsignedBigIntegerValueConverter {
                is_primary_key,
                column_name,
                field_name,
                entity_name,
            })
            .to_token_stream(),
            NumericType::Float(32) => (FloatValueConverter {
                is_primary_key,
                column_name,
                field_name,
                entity_name,
            })
            .to_token_stream(),
            NumericType::Float(64) => (DoubleValueConverter {
                is_primary_key,
                column_name,
                field_name,
                entity_name,
            })
            .to_token_stream(),
            _ => unreachable!(),
        }
    }

    pub fn converter_name(&self) -> TokenStream {
        let prefix = quote! {
            yukino::resolver::field_resolver_seeds
        };

        match self {
            NumericType::Integer(16) => quote! {
                #prefix::SmallIntegerValueConverter
            },
            NumericType::Integer(32) => quote! {
                #prefix::IntegerValueConverter
            },
            NumericType::Integer(64) => quote! {
                #prefix::BigIntegerValueConverter
            },
            NumericType::UnsignedInteger(16) => quote! {
                #prefix::UnsignedSmallIntegerValueConverter
            },
            NumericType::UnsignedInteger(32) => quote! {
                #prefix::UnsignedIntegerValueConverter
            },
            NumericType::UnsignedInteger(64) => quote! {
                #prefix::UnsignedBigIntegerValueConverter
            },
            NumericType::Float(32) => quote! {
                #prefix::FloatValueConverter
            },
            NumericType::Float(64) => quote! {
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

impl FieldResolverSeed for NumericFieldResolverSeed {
    fn new() -> Self
    where
        Self: Sized,
    {
        NumericFieldResolverSeed
    }

    fn boxed(&self) -> FieldResolverSeedBox {
        Box::new(NumericFieldResolverSeed)
    }

    fn try_breed(
        &self,
        entity_name: EntityName,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
        type_path_resolver: &TypePathResolver,
    ) -> Option<Result<FieldResolverBox, ResolveError>> {
        let (ty, field_type) = match field_type {
            Type::Path(type_path) => match type_path.path.segments.first() {
                Some(first_segment) => NumericType::from_ident(&first_segment.ident)
                    .map(|ty| (ty, type_path_resolver.get_full_path(type_path.clone()))),
                None => None,
            },
            _ => None,
        }?;

        let field = Self::default_annotations(annotations);

        let definition = ColumnDefinition {
            name: field
                .name
                .unwrap_or_else(|| ident.to_string().to_snake_case()),
            ty: ColumnType::NormalColumn(ident.to_string()),
            data_type: ty.database_type(),
            unique: field.unique,
            auto_increase: field.auto_increase,
            primary_key: Self::is_primary_key(annotations),
        };

        Some(Ok(Box::new(NumericFieldResolver {
            field_path: (entity_name, ident.to_string()),
            ty,
            definition,
            field_type,
        })))
    }
}

pub struct NumericFieldResolver {
    field_path: FieldPath,
    ty: NumericType,
    definition: ColumnDefinition,
    field_type: TypePath,
}

impl FieldResolver for NumericFieldResolver {
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
        let output_type = self.ty.converter_name();
        let converter = self.ty.converter_token_stream(
            self.definition.name.clone(),
            self.field_path.clone(),
            self.definition.primary_key,
        );

        let data_converter_token_stream = quote! {
            pub fn #method_name() -> #output_type {
                #converter
            }
        };

        let getter_name = self.getter_ident();
        let setter_name = self.setter_ident();
        let field_ident = format_ident!("{}", self.field_path().1);
        let field_type = &self.field_type;

        let field_getter_token_stream = quote! {
            pub fn #getter_name(&self) -> #field_type {
                let inner = self.get_inner();
                inner.#field_ident
            }
        };
        let field_setter_token_stream = quote! {
            pub fn #setter_name(&mut self, value: #field_type) -> &mut Self {
                let inner = self.get_inner_mut();
                inner.#field_ident= value;
                self
            }
        };

        Ok(AchievedFieldResolver {
            field_path: self.field_path.clone(),
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
            field_type: Type::Path(field_type.clone()),
        })
    }
}

macro_rules! impl_converter {
    ($ident: ident, $output_type: ty, $database_value: ident) => {
        #[derive(ToTokens)]
        #[Iroha(mod_path = "yukino::resolver::field_resolver_seeds")]
        pub struct $ident {
            is_primary_key: bool,
            column_name: String,
            entity_name: String,
            field_name: String,
        }

        impl ValueConverter<$output_type> for $ident {
            fn to_field_value(
                &self,
                values: &HashMap<String, DatabaseValue>,
            ) -> Result<$output_type, DataConvertError> {
                match values.get(&self.column_name) {
                    Some(DatabaseValue::$database_value(value)) => Ok(*value),
                    _ => Err(DataConvertError::UnexpectedDatabaseValueType(
                        self.entity_name.clone(),
                        self.field_name.clone(),
                    )),
                }
            }

            fn to_database_values_by_ref(
                &self,
                value: &$output_type,
            ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
                let mut map = HashMap::new();
                map.insert(
                    self.column_name.clone(),
                    DatabaseValue::$database_value(*value),
                );

                Ok(map)
            }

            fn primary_column_values_by_ref(
                &self,
                value: &$output_type,
            ) -> Result<HashMap<String, DatabaseValue>, DataConvertError> {
                if self.is_primary_key {
                    self.to_database_values_by_ref(value)
                } else {
                    Ok(HashMap::new())
                }
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
