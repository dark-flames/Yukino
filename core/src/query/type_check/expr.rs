use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Binary, ColumnIdent, Expr, Literal, Locatable, Unary};
use crate::query::type_check::{TypeChecker, TypeInfer};
use crate::types::{ExprWrapper, IdentResolveStatus, TypeInfo};

impl TypeInfer for Expr {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        match self {
            Expr::ColumnIdent(ident) => ident.try_wrap(ty_checker),
            Expr::Binary(binary) => binary.try_wrap(ty_checker),
            Expr::Unary(unary) => unary.try_wrap(ty_checker),
            Expr::Literal(lit) => lit.try_wrap(ty_checker),
            _ => unimplemented!(),
        }
    }

    fn wrap_with_ty<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
        type_info: TypeInfo,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        match self {
            Expr::ColumnIdent(ident) => ident.wrap_with_ty(ty_checker, type_info),
            Expr::Binary(binary) => binary.wrap_with_ty(ty_checker, type_info),
            Expr::Unary(unary) => unary.wrap_with_ty(ty_checker, type_info),
            Expr::Literal(lit) => lit.wrap_with_ty(ty_checker, type_info),
            _ => unimplemented!(),
        }
    }
}

impl TypeInfer for Binary {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        match (
            self.left.try_wrap(ty_checker)?,
            self.right.try_wrap(ty_checker)?,
        ) {
            (None, None) => Err(self.location().error(SyntaxError::TypeInferError)),
            (Some(left), Some(right)) => {
                let resolver_name = left.type_info.resolver_name.clone();
                let resolver = ty_checker.get_resolver(&resolver_name).ok_or_else(|| {
                    self.location()
                        .error(SyntaxError::UnknownResolverName(resolver_name))
                })?;
                if resolver.cmp_type_info(&left.type_info, &right.type_info) {
                    resolver
                        .handle_binary(left, right, self.location, self.operator)
                        .map(Some)
                } else {
                    Err(right.location().error(SyntaxError::TypeError(
                        left.type_info.field_type,
                        right.type_info.field_type,
                    )))
                }
            }
            (Some(left), None) => {
                let right = self
                    .right
                    .wrap_with_ty(ty_checker, left.type_info.clone())?;

                let resolver_name = left.type_info.resolver_name.clone();
                let resolver = ty_checker.get_resolver(&resolver_name).ok_or_else(|| {
                    self.location()
                        .error(SyntaxError::UnknownResolverName(resolver_name))
                })?;

                resolver
                    .handle_binary(left, right, self.location, self.operator)
                    .map(Some)
            }
            (None, Some(right)) => {
                let left = self
                    .left
                    .wrap_with_ty(ty_checker, right.type_info.clone())?;

                let resolver_name = right.type_info.resolver_name.clone();
                let resolver = ty_checker.get_resolver(&resolver_name).ok_or_else(|| {
                    self.location()
                        .error(SyntaxError::UnknownResolverName(resolver_name))
                })?;

                resolver
                    .handle_binary(left, right, self.location, self.operator)
                    .map(Some)
            }
        }
    }
}

impl TypeInfer for Unary {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        if let Some(right) = self.right.try_wrap(ty_checker)? {
            let resolver_name = right.type_info.resolver_name.clone();

            let resolver = ty_checker.get_resolver(&resolver_name).ok_or_else(|| {
                self.location()
                    .error(SyntaxError::UnknownResolverName(resolver_name))
            })?;

            resolver
                .handle_unary(right, self.location, self.operator)
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn wrap_with_ty<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
        type_info: TypeInfo,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        let right = self.right.wrap_with_ty(ty_checker, type_info)?;

        let resolver_name = right.type_info.resolver_name.clone();

        let resolver = ty_checker.get_resolver(&resolver_name).ok_or_else(|| {
            self.location()
                .error(SyntaxError::UnknownResolverName(resolver_name))
        })?;

        resolver.handle_unary(right, self.location, self.operator)
    }
}

impl TypeInfer for ColumnIdent {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        let mut ident = self.clone();

        loop {
            let table_alias = ident.segments.first().unwrap();

            let entity_name = ty_checker
                .get_table_name(self.segments.first().unwrap())
                .ok_or_else(|| {
                    self.location()
                        .error(SyntaxError::UnknownAlias(table_alias.to_string()))
                })?;

            let definition = ty_checker
                .get_field_definition(entity_name, self.segments.last().unwrap())
                .ok_or_else(|| {
                    self.location().error(SyntaxError::UnknownField(
                        entity_name.to_string(),
                        self.segments.last().unwrap().to_string(),
                    ))
                })?;

            let resolver = ty_checker
                .get_resolver(&definition.type_resolver_name)
                .ok_or_else(|| {
                    self.location().error(SyntaxError::UnknownResolverName(
                        definition.type_resolver_name.clone(),
                    ))
                })?;

            let (new_status, join_clauses) = resolver.wrap_ident(self, &definition)?;

            for join in join_clauses {
                ty_checker
                    .add_join_clause(join)
                    .map_err(|e| self.location().error(e))?;
            }

            match new_status {
                IdentResolveStatus::Unresolved(new_ident) => ident = new_ident,
                IdentResolveStatus::Resolved(wrapper) => break Ok(Some(wrapper)),
            }
        }
    }
}

impl TypeInfer for Literal {
    fn try_wrap<F>(
        &self,
        _ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        Ok(None)
    }

    fn wrap_with_ty<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
        type_info: TypeInfo,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        let resolver = ty_checker
            .get_resolver(&type_info.resolver_name)
            .ok_or_else(|| {
                self.location().error(SyntaxError::UnknownResolverName(
                    type_info.resolver_name.clone(),
                ))
            })?;

        let (expr, assertions) = resolver.wrap_lit(self, type_info)?;

        for (ident, ty) in assertions {
            ty_checker
                .add_external_value_assertion(ident, ty)
                .map_err(|e| self.location().error(e))?;
        }

        Ok(expr)
    }
}

#[test]
fn test_expr_type_check() {
    use crate::query::ast::*;
    use crate::query::grammar::*;
    use crate::query::type_check::{TypeChecker, TypeKind};
    use crate::resolver::field_resolver_seeds::NumericTypeResolver;
    use crate::types::TypeResolver;
    use pest::Parser;

    let expr1 = Expr::from_pair(
        Grammar::parse(Rule::expr, "t.a * 10 + 3 * t.b")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();

    let mut type_checker = TypeChecker::new(
        vec![NumericTypeResolver::seed()],
        vec![("t".to_string(), "test".to_string())]
            .into_iter()
            .collect(),
        |entity: &str, field: &str| match (entity, field) {
            ("test", "a") => Some(FieldDefinition {
                entity: "test".to_string(),
                name: "a".to_string(),
                type_resolver_name: "numeric".to_string(),
                field_type: "u64".to_string(),
                nullable: false,
                columns: vec![],
                tables: vec![],
                association: None,
            }),
            ("test", "b") => Some(FieldDefinition {
                entity: "test".to_string(),
                name: "b".to_string(),
                type_resolver_name: "numeric".to_string(),
                field_type: "u64".to_string(),
                nullable: false,
                columns: vec![],
                tables: vec![],
                association: None,
            }),
            _ => None,
        },
    );

    assert_eq!(
        expr1
            .try_wrap(&mut type_checker)
            .unwrap()
            .unwrap()
            .type_info,
        TypeInfo {
            resolver_name: "numeric".to_string(),
            field_type: "u64".to_string(),
            nullable: false,
            type_kind: TypeKind::Numeric
        }
    )
}
