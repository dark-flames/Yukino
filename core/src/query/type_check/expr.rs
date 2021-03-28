use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Binary, ColumnIdent, Expr, Literal, Locatable, Unary};
use crate::query::type_check::{TypeCheck, TypeChecker};
use crate::types::{ExprWrapper, TypeInfo};

impl TypeCheck for Expr {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> FieldDefinition,
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
        F: Fn(&str, &str) -> FieldDefinition,
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

impl TypeCheck for Binary {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> FieldDefinition,
    {
        match (
            self.left.try_wrap(ty_checker)?,
            self.right.try_wrap(ty_checker)?,
        ) {
            (None, None) => Err(self.location().error(SyntaxError::TypeInferError)),
            (Some(left), Some(right)) => {
                if left.type_info == right.type_info {
                    let resolver_name = left.type_info.resolver_name.clone();
                    let resolver = ty_checker.get_resolver(&resolver_name).ok_or_else(|| {
                        self.location()
                            .error(SyntaxError::UnknownResolverName(resolver_name))
                    })?;

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

impl TypeCheck for Unary {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> FieldDefinition,
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
        F: Fn(&str, &str) -> FieldDefinition,
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

impl TypeCheck for ColumnIdent {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> FieldDefinition,
    {
        assert_eq!(self.segments.len(), 2); // todo: support auto join and auto alias

        let table_alias = self.segments.first().unwrap();

        let definition = ty_checker.get_field_definition(
            ty_checker
                .get_table_name(self.segments.first().unwrap())
                .ok_or_else(|| {
                    self.location()
                        .error(SyntaxError::UnknownAlias(table_alias.to_string()))
                })?,
            self.segments.last().unwrap(),
        );

        let resolver = ty_checker
            .get_resolver(&definition.type_resolver_name)
            .ok_or_else(|| {
                self.location().error(SyntaxError::UnknownResolverName(
                    definition.type_resolver_name.clone(),
                ))
            })?;

        let type_info = TypeInfo {
            field_type: definition.field_type,
            nullable: definition.nullable,
            type_kind: resolver.type_kind(),
            resolver_name: definition.type_resolver_name.clone(),
        };

        Ok(Some(ExprWrapper {
            exprs: vec![Expr::ColumnIdent(self.clone())],
            type_info,
            location: self.location(),
        }))
    }
}

impl TypeCheck for Literal {
    fn try_wrap<F>(
        &self,
        _ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> FieldDefinition,
    {
        Ok(None)
    }

    fn wrap_with_ty<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
        type_info: TypeInfo,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> FieldDefinition,
    {
        let resolver = ty_checker
            .get_resolver(&type_info.resolver_name)
            .ok_or_else(|| {
                self.location().error(SyntaxError::UnknownResolverName(
                    type_info.resolver_name.clone(),
                ))
            })?;

        resolver.wrap_lit(self, type_info)
    }
}
