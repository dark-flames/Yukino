use proc_macro2::{TokenStream, Group};
use quote::{ToTokens, quote, TokenStreamExt};
use proc_macro2::Delimiter::Brace;

#[derive(Clone)]
#[allow(dead_code)]
pub enum DatabaseType {
    SmallInteger,
    UnsignedSmallInteger,
    Integer,
    UnsignedInteger,
    BigInteger,
    UnsignedBigInteger,

    Float,
    Double,

    #[cfg(any(feature="decimal"))]
    Decimal(usize, usize),

    Binary,

    #[cfg(any(feature="data-time"))]
    Time,
    #[cfg(any(feature="data-time"))]
    Date,
    #[cfg(any(feature="data-time"))]
    DateTime,

    Timestamp,

    Character,
    String,
    Text,

    Map,
    Array
}

impl ToTokens for DatabaseType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Group::new(
            Brace,
            match self {
                DatabaseType::SmallInteger => quote! {
                    yuikino::DatabaseType::SmallInteger
                },
                DatabaseType::UnsignedSmallInteger => quote! {
                    yuikino::DatabaseType::UnsignedSmallInteger
                },
                DatabaseType::Integer => quote! {
                    yuikino::DatabaseType::Integer
                },
                DatabaseType::UnsignedInteger => quote! {
                    yuikino::DatabaseType::UnsignedInteger
                },
                DatabaseType::BigInteger => quote! {
                    yuikino::DatabaseType::BigInteger
                },
                DatabaseType::UnsignedBigInteger => quote! {
                    yuikino::DatabaseType::UnsignedBigInteger
                },
                DatabaseType::Float => quote! {
                    yuikino::DatabaseType::Float
                },
                DatabaseType::Double => quote! {
                    yuikino::DatabaseType::Double
                },
                DatabaseType::Decimal(p, s) => quote! {
                    yuikino::DatabaseType::Decimal(#p, #s)
                },
                DatabaseType::Binary => quote! {
                    yuikino::DatabaseType::Binary
                },
                #[cfg(any(feature="data-time"))]
                DatabaseType::Time => quote! {
                    yuikino::DatabaseType::Time
                },
                #[cfg(any(feature="data-time"))]
                DatabaseType::Date => quote! {
                    yuikino::DatabaseType::Binary
                },
                #[cfg(any(feature="data-time"))]
                DatabaseType::DateTime => quote! {
                    yuikino::DatabaseType::Binary
                },
                DatabaseType::Timestamp => quote! {
                    yuikino::DatabaseType::Binary
                },
                DatabaseType::Character => quote! {
                    yuikino::DatabaseType::Character
                },
                DatabaseType::String => quote! {
                    yuikino::DatabaseType::String
                },
                DatabaseType::Text => quote! {
                    yuikino::DatabaseType::Text
                },
                DatabaseType::Array => quote! {
                    yuikino::DatabaseType::Array
                },
                DatabaseType::Map => quote! {
                    yuikino::DatabaseType::Map
                }
            }
        ))
    }
}

pub type Binary = Vec<u8>;

#[allow(dead_code)]
pub enum DatabaseValue {
    SmallInteger(i16),
    UnsignedSmallInteger(u16),
    Integer(i32),
    UnsignedInteger(u32),
    BigInteger(i64),
    UnsignedBigInteger(u64),

    Float(f32),
    Double(f64),

    #[cfg(any(feature="decimal"))]
    Decimal(rust_decimal::Decimal),

    Binary(Binary),

    #[cfg(any(feature="data-time"))]
    Time(chrono::Time),
    #[cfg(any(feature="data-time"))]
    Date(chrono::Date<dyn chore::TimeZone>),
    #[cfg(any(feature="data-time"))]
    DateTime(chrono::DateTime<dyn chore::TimeZone>),

    Timestamp(i64),

    Character(char),
    String(String),
    Text(String),

    BLOB(Binary),

    JSON(String)
}