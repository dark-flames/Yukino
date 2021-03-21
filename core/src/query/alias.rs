use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Expr, SelectClause, TableReference};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

pub const GLOBAL: &str = "#global_alias";

pub trait CollectTableAlias {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos>;
}

pub trait CollectExprAlias {
    fn collect_expr_alias(&self) -> Result<HashMap<String, Expr>, SyntaxErrorWithPos>;
}

impl CollectTableAlias for TableReference {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
        let alias = self.alias.clone().unwrap_or_else(|| GLOBAL.to_string());

        Ok(vec![(alias, self.name.clone())].into_iter().collect())
    }
}

#[allow(clippy::map_entry)]
impl CollectExprAlias for SelectClause {
    fn collect_expr_alias(&self) -> Result<HashMap<String, Expr, RandomState>, SyntaxErrorWithPos> {
        let items = self.items.iter().filter_map(|(expr, alias_option)| {
            alias_option.clone().map(|alias| (alias, expr.clone()))
        });

        let mut result = HashMap::new();

        for (alias, expr) in items {
            if result.contains_key(&alias) {
                return Err(self.location.error(SyntaxError::ConflictAlias(alias)));
            } else {
                result.insert(alias, expr);
            }
        }

        Ok(result)
    }
}
