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

    CLOB,
    BLOB,

    Map,
    List
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