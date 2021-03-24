use crate::types::DatabaseType;
use iroha::ToTokens;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(ToTokens, Clone, Eq, PartialEq, Debug)]
#[Iroha(mod_path = "yukino::query::type_check")]
pub enum TypeKind {
    Integer,
    Float,
    String,
    Boolean,
    Null,
    List(Box<TypeKind>),
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                TypeKind::Integer => "Integer".to_string(),
                TypeKind::Float => "Float".to_string(),
                TypeKind::String => "String".to_string(),
                TypeKind::Boolean => "Boolean".to_string(),
                TypeKind::Null => "Null".to_string(),
                TypeKind::List(nested) => format!("List({})", nested),
            }
        )
    }
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
            | DatabaseType::Character
            | DatabaseType::Json => TypeKind::String,
        }
    }
}
