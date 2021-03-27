use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{ColumnIdent, Expr, Locatable};
use crate::query::type_check::{TypeCheck, TypeChecker};
use crate::types::{ExprWrapper, TypeInfo};

impl TypeCheck for ColumnIdent {
    fn warp<F>(&self, ty_checker: &mut TypeChecker<F>) -> Result<ExprWrapper, SyntaxErrorWithPos>
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
        };

        Ok(ExprWrapper {
            exprs: vec![Expr::ColumnIdent(self.clone())],
            resolver_name: definition.type_resolver_name.clone(),
            type_info,
            location: self.location(),
        })
    }
}
