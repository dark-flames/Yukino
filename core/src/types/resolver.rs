use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{BinaryOperator, Expr, Literal, Locatable, Location, UnaryOperator};
use crate::query::type_check::TypeKind;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum CompareOperator {
    Bt,
    Bte,
    Lt,
    Lte,
    Neq,
    Eq,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TypeInfo {
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
    pub resolver_name: String,
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

    fn wrap_lit(
        &self,
        lit: &Literal,
        type_info: TypeInfo,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos>;

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
}

pub struct TypeResolverManager {
    resolvers: HashMap<String, Box<dyn TypeResolver>>,
}

impl TypeResolverManager {
    pub fn new(resolvers: Vec<Box<dyn TypeResolver>>) -> Self {
        TypeResolverManager {
            resolvers: resolvers
                .into_iter()
                .map(|resolver| (resolver.name(), resolver))
                .collect(),
        }
    }

    pub fn get_resolver(&self, name: &str) -> Option<&dyn TypeResolver> {
        self.resolvers.get(name).map(|boxed| boxed.as_ref())
    }
}
