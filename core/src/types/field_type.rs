use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Literal, Locatable};
use crate::query::type_check::{TypeKind, TypeChecker};
use quote::ToTokens;
use syn::Type;

pub type FieldTypeBox = Box<dyn FieldType>;
pub type TypeWrapperBox<T> = Box<dyn TypeWrapper<FieldType = T>>;

pub enum CompareOperator {
    Bt,
    Bte,
    Lt,
    Lte,
    Neq,
    Eq
}

pub trait TypeWrapper: Locatable {
    type FieldType: FieldType;

    fn add(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "add",
                Self::FieldType::name(),
            )))
    }

    fn minus(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "minus",
                Self::FieldType::name(),
            )))
    }

    fn mul(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "multi",
                Self::FieldType::name(),
            )))
    }

    fn div(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "div",
                Self::FieldType::name(),
            )))
    }

    fn rem(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "mod",
                Self::FieldType::name(),
            )))
    }

    fn left_shift(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "left_shift",
                Self::FieldType::name(),
            )))
    }

    fn right_shift(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "right_shift",
                Self::FieldType::name(),
            )))
    }

    fn bit_and(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_and",
                Self::FieldType::name(),
            )))
    }

    fn bit_or(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_or",
                Self::FieldType::name(),
            )))
    }

    fn bit_xor(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_xor",
                Self::FieldType::name(),
            )))
    }

    fn bit_reverse(self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_reverse",
                Self::FieldType::name(),
            )))
    }

    fn and(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "and",
                Self::FieldType::name(),
            )))
    }

    fn or(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "or",
                Self::FieldType::name(),
            )))
    }

    fn xor(&self, _others: &Self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "xor",
                Self::FieldType::name(),
            )))
    }

    fn not(&self) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "not",
                Self::FieldType::name(),
            )))
    }

    fn cmp(&self, _others: &Self, _ordering: CompareOperator) -> Result<Self, SyntaxErrorWithPos> where Self: Sized {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "compare",
                Self::FieldType::name(),
            )))
    }
}

pub trait FieldType: ToTokens {
    fn name() -> &'static str where Self: Sized;

    fn get_value_type() -> Type where Self: Sized;

    fn type_kind() -> TypeKind where Self: Sized;

    fn nullable() -> bool where Self: Sized;

    fn wrap_lit(lit: Literal, type_checker: &mut TypeChecker) -> Result<TypeWrapperBox<Self>, SyntaxErrorWithPos> where Self: Sized;
}
