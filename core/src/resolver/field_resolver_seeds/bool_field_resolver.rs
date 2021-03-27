use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{ColumnIdent, Expr};
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

    fn type_kind(&self) -> TypeKind {
        TypeKind::Boolean
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

    fn wrap_ident(
        &self,
        ident: &ColumnIdent,
        field_definition: &FieldDefinition,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        if &field_definition.field_type != "bool" {
            Err(ident.location().error(SyntaxError::TypeError(
                "bool".to_string(),
                field_definition.field_type.clone(),
            )))
        } else {
            let type_info = TypeInfo {
                field_type: field_definition.field_type.clone(),
                nullable: field_definition.nullable,
                type_kind: self.type_kind(),
            };

            Ok(ExprWrapper {
                exprs: vec![Expr::ColumnIdent(ident.clone())],
                resolver_name: self.name(),
                type_info,
                location: ident.location,
            })
        }
    }
}
