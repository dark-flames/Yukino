use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::Expr;
use crate::query::ast::{Literal, Locatable};
use crate::query::type_check::TypeKind;
use crate::types::{ExprWrapper, TypeInfo, TypeResolver};

pub struct BoolTypeResolver;

impl TypeResolver for BoolTypeResolver {
    fn seed() -> Box<dyn TypeResolver> {
        Box::new(BoolTypeResolver)
    }

    fn name(&self) -> String {
        "bool".to_string()
    }

    fn wrap_lit(
        &self,
        lit: &Literal,
        type_info: TypeInfo,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        match lit {
            Literal::Boolean(_) => Ok(ExprWrapper {
                exprs: vec![Expr::Literal(lit.clone())],
                resolver_name: self.name(),
                type_info,
                location: lit.location(),
            }),
            Literal::Null(_) if type_info.nullable => Ok(ExprWrapper {
                exprs: vec![Expr::Literal(lit.clone())],
                resolver_name: self.name(),
                type_info,
                location: lit.location(),
            }),
            _ => Err(lit.location().error(SyntaxError::TypeError(
                type_info.to_string(),
                TypeKind::from(lit).to_string(),
            ))),
        }
    }
}
