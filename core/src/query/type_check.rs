use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Boolean, Float, Integer, Str};
use crate::types::DatabaseType;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TypeKind {
    Integer,
    Float,
    String,
    Char,
    Boolean,
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f, "{}",
            match self {
                TypeKind::Integer => "Integer",
                TypeKind::Float => "Float",
                TypeKind::String => "String",
                TypeKind::Char => "Char",
                TypeKind::Boolean => "Boolean",
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
            | DatabaseType::Json => TypeKind::String,
            DatabaseType::Character => TypeKind::Char,
        }
    }
}

pub trait TypeCheck {
    fn assert_type(&self, type_kind: TypeKind) -> Result<(), SyntaxErrorWithPos>;

    fn infer_type(&self) -> Option<TypeKind>;
}

impl TypeCheck for Boolean {
    fn assert_type(&self, type_kind: TypeKind) -> Result<(), SyntaxErrorWithPos> {
        if type_kind == TypeKind::Boolean {
            Ok(())
        } else {
            Err(self
                .location
                .error(SyntaxError::TypeError(type_kind, TypeKind::Boolean)))
        }
    }

    fn infer_type(&self) -> Option<TypeKind> {
        Some(TypeKind::Boolean)
    }
}

impl TypeCheck for Integer {
    fn assert_type(&self, type_kind: TypeKind) -> Result<(), SyntaxErrorWithPos> {
        if type_kind == TypeKind::Integer {
            Ok(())
        } else {
            Err(self
                .location
                .error(SyntaxError::TypeError(type_kind, TypeKind::Integer)))
        }
    }

    fn infer_type(&self) -> Option<TypeKind> {
        Some(TypeKind::Integer)
    }
}

impl TypeCheck for Float {
    fn assert_type(&self, type_kind: TypeKind) -> Result<(), SyntaxErrorWithPos> {
        if type_kind == TypeKind::Float {
            Ok(())
        } else {
            Err(self
                .location
                .error(SyntaxError::TypeError(type_kind, TypeKind::Float)))
        }
    }

    fn infer_type(&self) -> Option<TypeKind> {
        Some(TypeKind::Float)
    }
}

impl TypeCheck for Str {
    fn assert_type(&self, type_kind: TypeKind) -> Result<(), SyntaxErrorWithPos> {
        if type_kind == TypeKind::String {
            Ok(())
        } else {
            Err(self
                .location
                .error(SyntaxError::TypeError(type_kind, TypeKind::String)))
        }
    }

    fn infer_type(&self) -> Option<TypeKind> {
        Some(TypeKind::String)
    }
}
