use crate::resolver::error::DataConvertError;
#[doc(hidden)]
use iroha::ToTokens;
#[doc(hidden)]
use serde_json::Value;
use std::collections::HashMap;
#[doc(hidden)]
#[cfg(any(feature = "data-time"))]
use time::{Date, PrimitiveDateTime, Time};

/// Field type in Yukino, may be different depends on platform or feature configuration
#[derive(Copy, Clone, ToTokens, Debug, Eq, PartialEq)]
#[Iroha(mod_path = "yukino::types")]
pub enum DatabaseType {
    SmallInteger,
    UnsignedSmallInteger,
    Integer,
    UnsignedInteger,
    BigInteger,
    UnsignedBigInteger,

    Float,
    Double,

    #[cfg(any(feature = "decimal"))]
    Decimal(u32),

    Binary,

    #[cfg(any(feature = "data-time"))]
    Time,
    #[cfg(any(feature = "data-time"))]
    Date,
    #[cfg(any(feature = "data-time"))]
    DateTime,

    Timestamp,

    Character,
    String,
    Text,

    #[cfg(any(feature = "json"))]
    Json,
}

/// Binary data, type alias of `Vec<u8>`
pub type Binary = Vec<u8>;

pub type ValuePack = HashMap<String, DatabaseValue>;

/// Raw data of database. It can be automatically converted to and from variables in the entity.
#[derive(Debug, Clone)]
pub enum DatabaseValue {
    SmallInteger(i16),
    UnsignedSmallInteger(u16),
    Integer(i32),
    UnsignedInteger(u32),
    BigInteger(i64),
    UnsignedBigInteger(u64),

    Float(f32),
    Double(f64),

    #[cfg(any(feature = "decimal"))]
    Decimal(rust_decimal::Decimal),

    Binary(Binary),

    #[cfg(any(feature = "data-time"))]
    Time(Time),
    #[cfg(any(feature = "data-time"))]
    Date(Date),
    #[cfg(any(feature = "data-time"))]
    DateTime(PrimitiveDateTime),

    Timestamp(u64),

    Character(char),
    String(String),
    Text(String),

    #[cfg(any(feature = "json"))]
    Json(Value),
}

impl DatabaseType {
    pub fn suitable_for_primary_key(&self) -> bool {
        !matches!(
            self,
            Self::Json
                | Self::Text
                | Self::DateTime
                | Self::Timestamp
                | Self::Date
                | Self::Time
                | Self::Binary
                | Self::Decimal(_)
                | Self::Double
                | Self::Float
        )
    }
}

impl From<&DatabaseValue> for DatabaseType {
    fn from(database_value: &DatabaseValue) -> Self {
        match database_value {
            DatabaseValue::SmallInteger(_) => DatabaseType::SmallInteger,
            DatabaseValue::UnsignedSmallInteger(_) => DatabaseType::UnsignedSmallInteger,
            DatabaseValue::Integer(_) => DatabaseType::UnsignedInteger,
            DatabaseValue::UnsignedInteger(_) => DatabaseType::UnsignedInteger,
            DatabaseValue::BigInteger(_) => DatabaseType::BigInteger,
            DatabaseValue::UnsignedBigInteger(_) => DatabaseType::UnsignedBigInteger,
            DatabaseValue::Float(_) => DatabaseType::Float,
            DatabaseValue::Double(_) => DatabaseType::Double,
            DatabaseValue::Decimal(value) => DatabaseType::Decimal(value.scale()),
            DatabaseValue::Binary(_) => DatabaseType::Binary,
            DatabaseValue::Time(_) => DatabaseType::Time,
            DatabaseValue::Date(_) => DatabaseType::Date,
            DatabaseValue::DateTime(_) => DatabaseType::DateTime,
            DatabaseValue::Timestamp(_) => DatabaseType::Timestamp,
            DatabaseValue::Character(_) => DatabaseType::Character,
            DatabaseValue::String(_) => DatabaseType::String,
            DatabaseValue::Text(_) => DatabaseType::Text,
            DatabaseValue::Json(_) => DatabaseType::Json,
        }
    }
}

impl DatabaseValue {
    pub fn hash_for_primary_key(&self) -> Result<String, DataConvertError> {
        let ty: DatabaseType = self.into();
        if ty.suitable_for_primary_key() {
            Ok(match self {
                DatabaseValue::SmallInteger(value) => value.to_string(),
                DatabaseValue::UnsignedSmallInteger(value) => value.to_string(),
                DatabaseValue::Integer(value) => value.to_string(),
                DatabaseValue::UnsignedInteger(value) => value.to_string(),
                DatabaseValue::BigInteger(value) => value.to_string(),
                DatabaseValue::UnsignedBigInteger(value) => value.to_string(),
                DatabaseValue::Character(value) => value.to_string(),
                DatabaseValue::String(value) => value.clone(),
                _ => return Err(DataConvertError::UnsuitableColumnDataTypeForPrimaryKey),
            })
        } else {
            Err(DataConvertError::UnsuitableColumnDataTypeForPrimaryKey)
        }
    }
}
