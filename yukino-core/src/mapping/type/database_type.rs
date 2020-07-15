use iroha::ToTokens;
use serde_json::Value;
#[cfg(any(feature = "data-time"))]
use time::{Date, PrimitiveDateTime, Time};

#[derive(Clone, ToTokens, Debug, Eq, PartialEq)]
#[allow(dead_code)]
#[Iroha(mod_path = "yukino::mapping")]
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
    Decimal(usize, usize),

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

pub type Binary = Vec<u8>;

#[allow(dead_code)]
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

    Timestamp(i64),

    Character(char),
    String(String),
    Text(String),

    #[cfg(any(feature = "json"))]
    Json(Value),
}
