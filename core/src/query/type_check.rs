use crate::types::DatabaseType;

pub enum TypeKind {
    Integer,
    Float,
    String,
    Char,
    Boolean,
}

impl From<DatabaseType> for TypeKind {
    fn from(db_type: DatabaseType) -> Self {
        match db_type {
            DatabaseType::Bool => TypeKind::Boolean,
            DatabaseType::SmallInteger
            | DatabaseType::UnsignedSmallInteger
            | DatabaseType::Integer
            | DatabaseType::UnsignedInteger
            | DatabaseType::BigInteger
            | DatabaseType::UnsignedBigInteger => TypeKind::Integer,
            DatabaseType::Float | DatabaseType::Double | DatabaseType::Decimal(_) => {
                TypeKind::Float
            }
            DatabaseType::Binary
            | DatabaseType::Time
            | DatabaseType::Date
            | DatabaseType::DateTime
            | DatabaseType::Timestamp
            | DatabaseType::String
            | DatabaseType::Text
            | DatabaseType::Json => TypeKind::String,
            DatabaseType::Character => TypeKind::Char,
        }
    }
}
