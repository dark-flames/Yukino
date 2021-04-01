use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    Binary, BinaryOperator, ColumnIdent, Expr, Location, Unary, UnaryOperator,
};
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
                type_info,
                location: lit.location(),
            }),
            Literal::Null(_) if type_info.nullable => Ok(ExprWrapper {
                exprs: vec![Expr::Literal(lit.clone())],
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
                resolver_name: self.name(),
                type_kind: self.type_kind(),
            };

            Ok(ExprWrapper {
                exprs: vec![Expr::ColumnIdent(ident.clone())],
                type_info,
                location: ident.location,
            })
        }
    }

    fn handle_binary(
        &self,
        mut left: ExprWrapper,
        mut right: ExprWrapper,
        location: Location,
        operator: BinaryOperator,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        let type_info = left.type_info.clone();

        if matches!(
            operator,
            BinaryOperator::BitXor
                | BinaryOperator::BitOr
                | BinaryOperator::BitAnd
                | BinaryOperator::Eq
                | BinaryOperator::Neq
                | BinaryOperator::And
                | BinaryOperator::Or
                | BinaryOperator::Xor
        ) {
            Ok(ExprWrapper {
                exprs: vec![Expr::Binary(Binary {
                    operator,
                    left: Box::new(left.exprs.pop().unwrap()),
                    right: Box::new(right.exprs.pop().unwrap()),
                    location,
                })],
                type_info,
                location,
            })
        } else {
            Err(location.error(SyntaxError::UnimplementedOperationForType(
                format!("{:?}", operator),
                left.type_info.to_string(),
            )))
        }
    }

    fn handle_unary(
        &self,
        mut item: ExprWrapper,
        location: Location,
        operator: UnaryOperator,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos> {
        Ok(ExprWrapper {
            exprs: vec![Expr::Unary(Unary {
                operator,
                right: Box::new(item.exprs.pop().unwrap()),
                location,
            })],
            type_info: item.type_info,
            location,
        })
    }
}
