use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    DeleteQuery, Expr, FromClause, GroupByClause, InsertQuery, JoinClause, Locatable,
    OrderByClause, Query, SelectQuery, UpdateQuery, ValueItem,
};
use crate::query::type_check::{TypeCheck, TypeChecker, TypeInfer, TypeKind};

impl TypeCheck for Query {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        match self {
            Query::Select(select) => select.check_type(ty_checker),
            Query::Delete(delete) => delete.check_type(ty_checker),
            Query::Update(update) => update.check_type(ty_checker),
            Query::Insert(insert) => insert.check_type(ty_checker),
        }
    }
}

impl TypeCheck for SelectQuery {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        self.from.check_type(ty_checker)?;

        if let Some(expr) = &mut self.where_clause {
            expr.check_type(ty_checker, Some(TypeKind::Boolean))?;
        }

        if let Some(group_by) = &mut self.group_by_clause {
            group_by.check_type(ty_checker)?;
        }

        if let Some(order_by) = &mut self.order_by_clause {
            order_by.check_type(ty_checker)?;
        }

        Ok(())
    }
}

impl TypeCheck for DeleteQuery {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        self.from.check_type(ty_checker)?;

        if let Some(expr) = &mut self.where_clause {
            expr.check_type(ty_checker, Some(TypeKind::Boolean))?;
        }

        Ok(())
    }
}

impl TypeCheck for UpdateQuery {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        for (_, value_item) in self.set_clause.items.iter_mut() {
            if let ValueItem::Expr(expr) = value_item {
                expr.check_type(ty_checker, None)?;
            }
        }

        if let Some(expr) = &mut self.where_clause {
            expr.check_type(ty_checker, Some(TypeKind::Boolean))?;
        }

        Ok(())
    }
}

impl TypeCheck for InsertQuery {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        for value in self.values.iter_mut() {
            value.check_type(ty_checker, None)?;
        }

        Ok(())
    }
}

impl TypeCheck for FromClause {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        for join in self.join.iter_mut() {
            if let JoinClause::JoinOn(join_on) = join {
                join_on.on.check_type(ty_checker, Some(TypeKind::Boolean))?;
            }
        }

        Ok(())
    }
}

impl TypeCheck for GroupByClause {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        self.by.check_type(ty_checker, None)?;

        if let Some(expr) = &mut self.having {
            expr.check_type(ty_checker, Some(TypeKind::Boolean))?;
        }

        Ok(())
    }
}

impl TypeCheck for OrderByClause {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        for (expr, _) in self.items.iter_mut() {
            expr.check_type(ty_checker, None)?;
        }

        Ok(())
    }
}

impl Expr {
    fn check_type<F>(
        &mut self,
        ty_checker: &mut TypeChecker<F>,
        type_kind: Option<TypeKind>,
    ) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        let wrapper = self
            .try_wrap(ty_checker)?
            .ok_or_else(|| self.location().error(SyntaxError::CannotInferType))?;

        if let Some(kind) = type_kind {
            if wrapper.type_info.type_kind != kind {
                return Err(wrapper.location().error(SyntaxError::TypeError(
                    kind.to_string(),
                    wrapper.type_info.type_kind.to_string(),
                )));
            }
        }

        *self = ty_checker
            .get_resolver(&wrapper.type_info.resolver_name)
            .ok_or_else(|| {
                wrapper.location().error(SyntaxError::UnknownResolverName(
                    wrapper.type_info.resolver_name.clone(),
                ))
            })?
            .unwrap_expr(wrapper)?;

        Ok(())
    }
}
