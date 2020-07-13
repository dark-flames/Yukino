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
use quote::ToTokens;
use std::collections::HashMap;
use syn::Type;

#[derive(ToTokens)]
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

    pub fn get_converter_token_stream(&self, column_name: &String) -> TokenStream {
        match self {
            NumericType::Integer(16) => (SmallIntegerValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            NumericType::Integer(32) => (IntegerValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            NumericType::Integer(64) => (BigIntegerValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(16) => (UnsignedSmallIntegerValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(32) => (UnsignedIntegerValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            NumericType::UnsignedInteger(64) => (UnsignedBigIntegerValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            NumericType::Float(32) => (FloatValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            NumericType::Float(64) => (DoubleValueConverter {
                column_name: column_name.clone(),
            })
            .to_token_stream(),
            _ => unreachable!(),
        }
    }

    pub fn get_converter_name(&self) -> TokenStream {
        let prefix = quote::quote! {
            yukino::mapping::resolver
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

    pub fn get_database_type(&self) -> DatabaseType {
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

macro_rules! impl_converter {
    ($ident: ident, $output_type: ty, $database_value: ident) => {
        impl ValueConverter<$output_type> for $ident {
            fn to_value(
                &self,
                values: &HashMap<String, DatabaseValue>,
            ) -> Result<$output_type, ParseError> {
                match values.get(&self.column_name) {
                    Some(DatabaseValue::$database_value(value)) => Ok(*value),
                    _ => {
                        let message =
                            format!("Unexpected DatabaseValue on field {}", &self.column_name);
                        Err(ParseError::new(&message))
                    }
                }
            }

            fn to_database_value(
                &self,
                value: &$output_type,
            ) -> Result<HashMap<String, DatabaseValue>, ParseError> {
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

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct SmallIntegerValueConverter {
    column_name: String,
}

impl_converter!(SmallIntegerValueConverter, i16, SmallInteger);

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct IntegerValueConverter {
    column_name: String,
}
impl_converter!(IntegerValueConverter, i32, Integer);

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct BigIntegerValueConverter {
    column_name: String,
}
impl_converter!(BigIntegerValueConverter, i64, BigInteger);

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct UnsignedSmallIntegerValueConverter {
    column_name: String,
}
impl_converter!(
    UnsignedSmallIntegerValueConverter,
    u16,
    UnsignedSmallInteger
);

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct UnsignedIntegerValueConverter {
    column_name: String,
}
impl_converter!(UnsignedIntegerValueConverter, u32, UnsignedInteger);

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct UnsignedBigIntegerValueConverter {
    column_name: String,
}
impl_converter!(UnsignedBigIntegerValueConverter, u64, UnsignedBigInteger);

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct FloatValueConverter {
    column_name: String,
}
impl_converter!(FloatValueConverter, f32, Float);

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::mapping::resolver")]
pub struct DoubleValueConverter {
    column_name: String,
}
impl_converter!(DoubleValueConverter, f64, Double);

pub struct NumericResolveCell {
    status: FieldResolveStatus,
    entity_name: Option<String>,
    field_ident: Option<Ident>,
    is_primary_key: Option<bool>,
    column: Option<Column>,
    ty: Option<NumericType>,
}

impl ConstructableCell for NumericResolveCell {
    fn get_seed() -> Self
    where
        Self: Sized,
    {
        NumericResolveCell {
            status: FieldResolveStatus::Seed,
            entity_name: None,
            field_ident: None,
            is_primary_key: None,
            column: None,
            ty: None,
        }
    }
}

impl FieldResolveCell for NumericResolveCell {
    fn weight(&self) -> usize {
        1
    }

    fn get_status(&self) -> FieldResolveStatus {
        self.status.clone()
    }

    fn resolve_fields(
        &mut self,
        _fields: HashMap<FieldPath, &dyn FieldResolveCell>,
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
        unimplemented!()
    }

    fn field_name(&self) -> Result<String, UnresolvedError> {
        self.field_ident
            .as_ref()
            .map(|name| name.to_string())
            .ok_or_else(|| UnresolvedError::new("Integer resolve cell"))
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
        self.is_primary_key
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))
    }

    fn get_foreigner_keys(&self) -> Result<Vec<ForeignKeyDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Finished => Ok(vec![]),
            _ => Err(UnresolvedError::new("Integer Resolve cell")),
        }
    }

    fn get_column_definitions(&self) -> Result<Vec<ColumnDefinition>, UnresolvedError> {
        match self.status {
            FieldResolveStatus::Finished => Ok(vec![ColumnDefinition {
                name: self.column_names()?[0].clone(),
                column_type: self.ty.as_ref().unwrap().get_database_type(),
                unique: self
                    .column
                    .as_ref()
                    .map(|column| column.unique)
                    .unwrap_or(false),
                auto_increase: self
                    .column
                    .as_ref()
                    .map(|column| column.auto_increase)
                    .unwrap_or(false),
                is_primary_key: self.is_primary_key.unwrap(),
            }]),
            _ => Err(UnresolvedError::new("Integer Resolve cell")),
        }
    }

    fn get_joined_table_definitions(&self) -> Result<Vec<TableDefinition>, UnresolvedError> {
        Ok(vec![])
    }

    fn get_data_converter_token_stream(&self) -> Result<TokenStream, UnresolvedError> {
        let column_name = self.column_names()?[0].clone();
        let converter = self
            .ty
            .as_ref()
            .ok_or_else(|| UnresolvedError::new("Integer Resolve cell"))?
            .get_converter_token_stream(&column_name);

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
            Type::Path(type_path) => match type_path.path.segments.first() {
                Some(first_segment) => NumericType::from_ident(&first_segment.ident),
                None => None,
            },
            _ => None,
        }
        .ok_or_else(|| {
            let message = format!(
                "{} is not supported by integer resolve cell",
                field_type.to_token_stream().to_string()
            );
            ResolveError::new(&entity_name, &message)
        })?;

        let is_primary_key = attributes.iter().any(|attr| {
            if let FieldAttribute::Id(_) = attr {
                true
            } else {
                false
            }
        });

        let column = attributes
            .iter()
            .filter_map(|attr| match attr {
                FieldAttribute::Column(column) => Some(column.clone()),
                _ => None,
            })
            .next();

        Ok(Box::new(NumericResolveCell {
            status: FieldResolveStatus::Finished,
            entity_name: Some(entity_name),
            field_ident: Some(ident.clone()),
            is_primary_key: Some(is_primary_key),
            column,
            ty: Some(ty),
        }))
    }

    fn match_field(&self, _attributes: &[FieldAttribute], field_type: &Type) -> bool {
        match field_type {
            Type::Path(type_path) => match type_path.path.segments.first() {
                Some(first_segment) => NumericType::from_ident(&first_segment.ident).is_some(),
                None => false,
            },
            _ => false,
        }
    }
}
