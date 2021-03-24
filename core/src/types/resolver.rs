use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Expr, Literal, Locatable, Location};
use std::collections::HashMap;

pub enum CompareOperator {
    Bt,
    Bte,
    Lt,
    Lte,
    Neq,
    Eq,
}

pub struct ExprWrapper {
    pub exprs: Vec<Expr>,
    pub resolver_name: String,
    pub field_type: String,
    pub location: Location,
}

impl Locatable for ExprWrapper {
    fn location(&self) -> Location {
        self.location
    }
}

pub trait TypeResolver {
    fn name(&self) -> String;

    fn wrap_lit(&self, lit: &Literal) -> Result<ExprWrapper, SyntaxErrorWithPos>;

    fn add(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "add",
                left.field_type,
            )))
    }

    fn minus(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "minus",
                left.field_type,
            )))
    }

    fn multi(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "multi",
                left.field_type,
            )))
    }

    fn div(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "div",
                left.field_type,
            )))
    }

    fn modulo(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "mod",
                left.field_type,
            )))
    }

    fn left_shift(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "left_shift",
                left.field_type,
            )))
    }

    fn right_shift(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "right_shift",
                left.field_type,
            )))
    }

    fn bit_and(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_and",
                left.field_type,
            )))
    }

    fn bit_or(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_or",
                left.field_type,
            )))
    }

    fn bit_xor(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_xor",
                left.field_type,
            )))
    }

    fn bit_reverse(&self, item: ExprWrapper) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(item
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "bit_reverse",
                item.field_type,
            )))
    }

    fn and(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "and",
                left.field_type,
            )))
    }

    fn or(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "or",
                left.field_type,
            )))
    }

    fn xor(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "xor",
                left.field_type,
            )))
    }

    fn not(&self, item: ExprWrapper) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(item
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "xor",
                item.field_type,
            )))
    }

    fn cmp(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
        _operator: CompareOperator,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(left
            .location()
            .error(SyntaxError::UnimplementedOperationForType(
                "compare",
                left.field_type,
            )))
    }
}

pub struct TypeResolverManager {
    pub resolvers: HashMap<String, Box<dyn TypeResolver>>,
}
