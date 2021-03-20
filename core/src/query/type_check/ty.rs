use crate::types::DatabaseType;

pub enum QueryType {
    Integer,
    Float,
    String,
    Char,
    Boolean,
}

impl From<DatabaseType> for QueryType {
    fn from(db_type: DatabaseType) -> Self {
        match db_type {
            DatabaseType::Bool => QueryType::Boolean,
            DatabaseType::SmallInteger
            | DatabaseType::UnsignedSmallInteger
            | DatabaseType::Integer
            | DatabaseType::UnsignedInteger
            | DatabaseType::BigInteger
            | DatabaseType::UnsignedBigInteger => QueryType::Integer,
            DatabaseType::Float | DatabaseType::Double | DatabaseType::Decimal(_) => {
                QueryType::Float
            }
            DatabaseType::Binary
            | DatabaseType::Time
            | DatabaseType::Date
            | DatabaseType::DateTime
            | DatabaseType::Timestamp
            | DatabaseType::String
            | DatabaseType::Text
            | DatabaseType::Json => QueryType::String,
            DatabaseType::Character => QueryType::Char,
        }
    }
}
