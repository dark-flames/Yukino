use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    BinaryOperator, ColumnIdent, Expr, JoinClause, Literal, Locatable, Location, UnaryOperator,
};
use crate::query::type_check::TypeKind;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum IdentResolveStatus {
    Unresolved(ColumnIdent),
    Resolved(ExprWrapper),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TypeInfo {
    pub resolver_name: String,
    pub field_type: String,
    pub nullable: bool,
    pub type_kind: TypeKind,
}

impl Display for TypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.nullable {
            write!(f, "Option<{}>", self.field_type)
        } else {
            write!(f, "{}", self.field_type)
        }
    }
}

pub struct ExprWrapper {
    pub exprs: Vec<Expr>,
    pub type_info: TypeInfo,
    pub location: Location,
}

impl Locatable for ExprWrapper {
    fn location(&self) -> Location {
        self.location
    }
}

pub trait TypeResolver {
    fn seed() -> Box<dyn TypeResolver>
    where
        Self: Sized;

    fn name(&self) -> String;

    fn cmp_type_info(&self, a: &TypeInfo, b: &TypeInfo) -> bool {
        a == b
    }

    fn wrap_lit(
        &self,
        lit: &Literal,
        type_info: TypeInfo,
    ) -> Result<(ExprWrapper, Vec<(String, String)>), SyntaxErrorWithPos>;

    fn wrap_ident(
        &self,
        ident: &ColumnIdent,
        field_definition: &FieldDefinition,
    ) -> Result<(IdentResolveStatus, Vec<JoinClause>), SyntaxErrorWithPos>;

    fn handle_binary(
        &self,
        left: ExprWrapper,
        _right: ExprWrapper,
        location: Location,
        operator: BinaryOperator,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(location.error(SyntaxError::UnimplementedOperationForType(
            format!("{:?}", operator),
            left.type_info.to_string(),
        )))
    }

    fn handle_unary(
        &self,
        item: ExprWrapper,
        location: Location,
        operator: UnaryOperator,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Err(location.error(SyntaxError::UnimplementedOperationForType(
            format!("{:?}", operator),
            item.type_info.to_string(),
        )))
    }

    fn unwrap_expr(&self, mut wrapper: ExprWrapper) -> Result<Expr, SyntaxErrorWithPos> {
        Ok(wrapper.exprs.pop().unwrap())
    }
}
