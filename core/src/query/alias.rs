use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    DeleteQuery, Expr, FromClause, JoinClause, Locatable, Location, Query, SelectClause,
    SelectQuery, TableReference, UpdateQuery,
};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

pub const GLOBAL: &str = "#global_alias";

pub trait CollectTableAlias {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos>;
}

pub trait CollectExprAlias {
    fn collect_expr_alias(&self) -> Result<HashMap<String, Expr>, SyntaxErrorWithPos>;
}

#[allow(clippy::map_entry)]
fn merge_hash_map(
    a: Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos>,
    b: Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos>,
    location: Location,
) -> Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos> {
    let mut result = a?;
    for (alias, table) in b? {
        if result.contains_key(&alias) {
            return Err(location.error(SyntaxError::ConflictAlias(alias)));
        } else {
            result.insert(alias, table);
        }
    }

    Ok(result)
}

impl CollectTableAlias for TableReference {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
        let alias = self.alias.clone().unwrap_or_else(|| GLOBAL.to_string());

        Ok(vec![(alias, self.name.clone())].into_iter().collect())
    }
}

impl CollectTableAlias for JoinClause {
    fn collect_table_alias(
        &self,
    ) -> Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos> {
        match self {
            JoinClause::NaturalJoin(nature_join) => nature_join.table.collect_table_alias(),
            JoinClause::JoinOn(join_on) => join_on.table.collect_table_alias(),
            JoinClause::CrossJoin(cross_join) => cross_join.table.collect_table_alias(),
        }
    }
}

impl CollectTableAlias for FromClause {
    fn collect_table_alias(
        &self,
    ) -> Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos> {
        let result = self.table.collect_table_alias();

        self.join
            .iter()
            .map(|j| (j.collect_table_alias(), j.location()))
            .fold(result, |carry, (result, location)| {
                merge_hash_map(carry, result, location)
            })
    }
}

impl CollectTableAlias for SelectQuery {
    fn collect_table_alias(
        &self,
    ) -> Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos> {
        self.from.collect_table_alias()
    }
}

impl CollectTableAlias for DeleteQuery {
    fn collect_table_alias(
        &self,
    ) -> Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos> {
        self.from.collect_table_alias()
    }
}

impl CollectTableAlias for UpdateQuery {
    fn collect_table_alias(
        &self,
    ) -> Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos> {
        let result = self.update_table.collect_table_alias();

        if let Some(from) = self.from_table.as_ref() {
            merge_hash_map(result, from.collect_table_alias(), from.location)
        } else {
            result
        }
    }
}

impl CollectTableAlias for Query {
    fn collect_table_alias(
        &self,
    ) -> Result<HashMap<String, String, RandomState>, SyntaxErrorWithPos> {
        match self {
            Query::Select(select) => select.collect_table_alias(),
            Query::Delete(delete) => delete.collect_table_alias(),
            Query::Update(update) => update.collect_table_alias(),
            Query::Insert(_) => Ok(HashMap::new()),
        }
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
