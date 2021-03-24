use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Literal, Locatable};
use crate::query::type_check::{TypeChecker, TypeKind};
use quote::ToTokens;
use syn::Type;

pub type FieldTypeBox = Box<dyn FieldType>;
pub type TypeWrapperBox = Box<dyn TypeWrapper>;

pub enum CompareOperator {
    Bt,
    Bte,
    Lt,
    Lte,
    Neq,
    Eq,
}

pub trait TypeWrapper: Locatable {
    fn field_type(&self) -> FieldTypeBox;

    fn add(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "add",
                self.field_type().name(),
            )))
    }

    fn minus(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "minus",
                self.field_type().name(),
            )))
    }

    fn mul(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "multi",
                self.field_type().name(),
            )))
    }

    fn div(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "div",
                self.field_type().name(),
            )))
    }

    fn rem(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "mod",
                self.field_type().name(),
            )))
    }

    fn left_shift(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "left_shift",
                self.field_type().name(),
            )))
    }

    fn right_shift(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "right_shift",
                self.field_type().name(),
            )))
    }

    fn bit_and(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_and",
                self.field_type().name(),
            )))
    }

    fn bit_or(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_or",
                self.field_type().name(),
            )))
    }

    fn bit_xor(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_xor",
                self.field_type().name(),
            )))
    }

    fn bit_reverse(self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_reverse",
                self.field_type().name(),
            )))
    }

    fn and(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "and",
                self.field_type().name(),
            )))
    }

    fn or(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "or",
                self.field_type().name(),
            )))
    }

    fn xor(&self, _others: &Self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "xor",
                self.field_type().name(),
            )))
    }

    fn not(&self) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "not",
                self.field_type().name(),
            )))
    }

    fn cmp(
        &self,
        _others: &Self,
        _ordering: CompareOperator,
    ) -> Result<TypeWrapperBox, SyntaxErrorWithPos>
    where
        Self: Sized,
    {
        Err(self
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "compare",
                self.field_type().name(),
            )))
    }
}

pub trait FieldType: ToTokens {
    fn name(&self) -> &'static str;

    fn get_value_type(&self) -> Type;

    fn type_kind(&self) -> TypeKind;

    fn nullable(&self) -> bool;

    fn wrap_lit(
        &self,
        lit: Literal,
        type_checker: &mut TypeChecker,
    ) -> Result<FieldTypeBox, SyntaxErrorWithPos>;
}
