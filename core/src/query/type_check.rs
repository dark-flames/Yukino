use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Boolean, ExternalValue, Float, Integer, Literal, Null, Str};
use crate::types::DatabaseType;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TypeKind {
    Integer,
    Float,
    String,
    Char,
    Boolean,
    Null,
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                TypeKind::Integer => "Integer",
                TypeKind::Float => "Float",
                TypeKind::String => "String",
                TypeKind::Char => "Char",
                TypeKind::Boolean => "Boolean",
                TypeKind::Null => "Null",
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

pub struct TypeChecker {
    external_value_assertion: HashMap<String, TypeKind>,
}

#[allow(clippy::map_entry)]
impl TypeChecker {
    pub fn add_external_value_assertion(
        &mut self,
        ident: String,
        ty: TypeKind,
    ) -> Result<(), SyntaxError> {
        if self.external_value_assertion.contains_key(&ident) {
            Err(SyntaxError::ConflictValueAssertion(ident))
        } else {
            self.external_value_assertion.insert(ident, ty);
            Ok(())
        }
    }
}

pub trait TypeCheck {
    fn assert_type(
        &self,
        type_kind: TypeKind,
        type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos>;

    fn infer_type(&self, type_checker: &mut TypeChecker) -> Option<TypeKind>;
}

impl TypeCheck for Boolean {
    fn assert_type(
        &self,
        type_kind: TypeKind,
        _type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos> {
        if type_kind == TypeKind::Boolean {
            Ok(())
        } else {
            Err(self
                .location
                .error(SyntaxError::TypeError(type_kind, TypeKind::Boolean)))
        }
    }

    fn infer_type(&self, _type_checker: &mut TypeChecker) -> Option<TypeKind> {
        Some(TypeKind::Boolean)
    }
}

impl TypeCheck for Integer {
    fn assert_type(
        &self,
        type_kind: TypeKind,
        _type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos> {
        if type_kind == TypeKind::Integer {
            Ok(())
        } else {
            Err(self
                .location
                .error(SyntaxError::TypeError(type_kind, TypeKind::Integer)))
        }
    }

    fn infer_type(&self, _type_checker: &mut TypeChecker) -> Option<TypeKind> {
        Some(TypeKind::Integer)
    }
}

impl TypeCheck for Float {
    fn assert_type(
        &self,
        type_kind: TypeKind,
        _type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos> {
        if type_kind == TypeKind::Float {
            Ok(())
        } else {
            Err(self
                .location
                .error(SyntaxError::TypeError(type_kind, TypeKind::Float)))
        }
    }

    fn infer_type(&self, _type_checker: &mut TypeChecker) -> Option<TypeKind> {
        Some(TypeKind::Float)
    }
}

impl TypeCheck for Str {
    fn assert_type(
        &self,
        type_kind: TypeKind,
        _type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos> {
        if type_kind == TypeKind::String {
            Ok(())
        } else {
            Err(self
                .location
                .error(SyntaxError::TypeError(type_kind, TypeKind::String)))
        }
    }

    fn infer_type(&self, _type_checker: &mut TypeChecker) -> Option<TypeKind> {
        Some(TypeKind::String)
    }
}

impl TypeCheck for ExternalValue {
    fn assert_type(
        &self,
        type_kind: TypeKind,
        type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos> {
        type_checker
            .add_external_value_assertion(self.ident.clone(), type_kind)
            .map_err(|e| self.location.error(e))
    }

    fn infer_type(&self, _type_checker: &mut TypeChecker) -> Option<TypeKind> {
        None
    }
}

impl TypeCheck for Null {
    fn assert_type(
        &self,
        _type_kind: TypeKind,
        _type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos> {
        unimplemented!()
    }

    fn infer_type(&self, _type_checker: &mut TypeChecker) -> Option<TypeKind> {
        unimplemented!()
    }
}

impl TypeCheck for Literal {
    fn assert_type(
        &self,
        type_kind: TypeKind,
        type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos> {
        match self {
            Literal::Boolean(b) => b.assert_type(type_kind, type_checker),
            Literal::Integer(i) => i.assert_type(type_kind, type_checker),
            Literal::Float(f) => f.assert_type(type_kind, type_checker),
            Literal::String(s) => s.assert_type(type_kind, type_checker),
            Literal::External(e) => e.assert_type(type_kind, type_checker),
            Literal::Null(n) => n.assert_type(type_kind, type_checker),
        }
    }

    fn infer_type(&self, type_checker: &mut TypeChecker) -> Option<TypeKind> {
        match self {
            Literal::Boolean(b) => b.infer_type(type_checker),
            Literal::Integer(i) => i.infer_type(type_checker),
            Literal::Float(f) => f.infer_type(type_checker),
            Literal::String(s) => s.infer_type(type_checker),
            Literal::External(e) => e.infer_type(type_checker),
            Literal::Null(n) => n.infer_type(type_checker),
        }
    }
}
