use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{DeleteQuery, Expr, FromClause, JoinClause, Locatable, Location, Query, SelectClause, SelectQuery, TableReference, UpdateQuery, GroupByClause, OrderByClause};
use std::collections::{HashMap, HashSet};

pub const GLOBAL: &str = "#global_alias";

pub trait CollectTableAlias {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos>;
}

pub trait CollectExprAlias {
    fn collect_expr_alias(&self) -> Result<HashMap<String, Expr>, SyntaxErrorWithPos>;
}

#[allow(clippy::map_entry)]
fn merge_hash_map(
    a: Result<HashMap<String, String>, SyntaxErrorWithPos>,
    b: Result<HashMap<String, String>, SyntaxErrorWithPos>,
    location: Location,
) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
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
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
        match self {
            JoinClause::NaturalJoin(nature_join) => nature_join.table.collect_table_alias(),
            JoinClause::JoinOn(join_on) => join_on.table.collect_table_alias(),
            JoinClause::CrossJoin(cross_join) => cross_join.table.collect_table_alias(),
        }
    }
}

impl CollectTableAlias for FromClause {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
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
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
        self.from.collect_table_alias()
    }
}

impl CollectTableAlias for DeleteQuery {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
        self.from.collect_table_alias()
    }
}

impl CollectTableAlias for UpdateQuery {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
        let result = self.update_table.collect_table_alias();

        if let Some(from) = self.from_table.as_ref() {
            merge_hash_map(result, from.collect_table_alias(), from.location)
        } else {
            result
        }
    }
}

impl CollectTableAlias for Query {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
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
    fn collect_expr_alias(&self) -> Result<HashMap<String, Expr>, SyntaxErrorWithPos> {
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

impl CollectExprAlias for SelectQuery {
    fn collect_expr_alias(&self) -> Result<HashMap<String, Expr>, SyntaxErrorWithPos> {
        self.select_clause.collect_expr_alias()
    }
}

impl CollectExprAlias for Query {
    fn collect_expr_alias(&self) -> Result<HashMap<String, Expr>, SyntaxErrorWithPos> {
        match self {
            Query::Select(select) => select.collect_expr_alias(),
            _ => Ok(HashMap::new()),
        }
    }
}
pub trait ReplaceIdent {
    fn replace(
        &mut self,
        generated_alias: &HashMap<String, String>,
        table_alias: &HashMap<String, String>,
        field_map: &HashMap<String, HashSet<String>>
    ) -> Result<(), SyntaxErrorWithPos>;
}

impl ReplaceIdent for Expr {
    fn replace(
        &mut self,
        generated_alias: &HashMap<String, String>,
        table_alias: &HashMap<String, String>,
        field_map: &HashMap<String, HashSet<String>>
    ) -> Result<(), SyntaxErrorWithPos> {
        match self {
            Expr::ColumnIdent(ident) => {
                let first_segment = ident.segments.first().unwrap();

                if !table_alias.contains_key(first_segment) {
                    let mut entities: Vec<_> = field_map.iter().filter_map(
                        |(entity_name, field_map)| {
                            if field_map.contains(first_segment) {
                                Some(entity_name.clone())
                            } else {
                                None
                            }
                        }
                    ).collect();

                    let alias = if entities.len() == 1 {
                        let entity_name = entities.pop().unwrap();

                        generated_alias.get(&entity_name).ok_or_else(
                            || ident.location().error(SyntaxError::UnknownField(
                                "*".to_string(),
                                first_segment.clone()
                            ))
                        )?.clone()
                    } else {
                        return Err(ident.location().error(SyntaxError::ConflictAlias(first_segment.clone())))
                    };

                    ident.segments.insert(0, alias);
                };

                Ok(())
            },
            Expr::Binary(binary) => {
                binary.left.replace(generated_alias, table_alias, field_map)?;
                binary.right.replace(generated_alias, table_alias, field_map)
            },
            Expr::Unary(unary) => {
                unary.right.replace(generated_alias, table_alias, field_map)
            },
            _ => Ok(())
        }
    }
}

impl ReplaceIdent for SelectClause {
    fn replace(
        &mut self,
        generated_alias: &HashMap<String, String>,
        table_alias: &HashMap<String, String>,
        field_map: &HashMap<String, HashSet<String>>
    ) -> Result<(), SyntaxErrorWithPos> {
        for (expr, _) in self.items.iter_mut() {
            expr.replace(generated_alias, table_alias, field_map)?;
        }

        Ok(())
    }
}

impl ReplaceIdent for FromClause {
    fn replace(
        &mut self,
        generated_alias: &HashMap<String, String>,
        table_alias: &HashMap<String, String>,
        field_map: &HashMap<String, HashSet<String>>
    ) -> Result<(), SyntaxErrorWithPos> {
        for join in self.join.iter_mut() {
            if let JoinClause::JoinOn(join_on) = join {
                join_on.on.replace(generated_alias, table_alias, field_map)?;
            };
        }
        Ok(())
    }
}

impl ReplaceIdent for GroupByClause {
    fn replace(
        &mut self,
        generated_alias: &HashMap<String, String>,
        table_alias: &HashMap<String, String>,
        field_map: &HashMap<String, HashSet<String>>
    ) -> Result<(), SyntaxErrorWithPos> {
        self.by.replace(generated_alias, table_alias, field_map)?;

        if let Some(having) = &mut self.having {
            having.replace(generated_alias, table_alias, field_map)?;
        };

        Ok(())
    }
}

impl ReplaceIdent for OrderByClause {
    fn replace(
        &mut self,
        generated_alias: &HashMap<String, String>,
        table_alias: &HashMap<String, String>,
        field_map: &HashMap<String, HashSet<String>>
    ) -> Result<(), SyntaxErrorWithPos> {
        for (expr, _) in self.items.iter_mut() {
            expr.replace(generated_alias, table_alias, field_map)?;
        }

        Ok(())
    }
}

impl ReplaceIdent for SelectQuery {
    fn replace(
        &mut self,
        generated_alias: &HashMap<String, String>,
        table_alias: &HashMap<String, String>,
        field_map: &HashMap<String, HashSet<String>>
    ) -> Result<(), SyntaxErrorWithPos> {
        self.select_clause.replace(generated_alias, table_alias, field_map)?;
        self.from.replace(generated_alias, table_alias, field_map)?;

        if let Some(where_clause) = &mut self.where_clause {
            where_clause.replace(generated_alias, table_alias, field_map)?;
        }

        if let Some(group) = &mut self.group_by_clause {
            group.replace(generated_alias, table_alias, field_map)?;
        }

        if let Some(order_by_clause) = &mut self.order_by_clause {
            order_by_clause.replace(generated_alias, table_alias, field_map)?;
        }

        Ok(())
    }
}
