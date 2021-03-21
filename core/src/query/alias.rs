use crate::query::ast::error::SyntaxErrorWithPos;
use crate::query::ast::TableReference;
use std::collections::HashMap;

pub const GLOBAL: &str = "#global_alias";

pub trait CollectTableAlias {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos>;
}

impl CollectTableAlias for TableReference {
    fn collect_table_alias(&self) -> Result<HashMap<String, String>, SyntaxErrorWithPos> {
        let alias = self.alias.clone().unwrap_or_else(|| GLOBAL.to_string());

        Ok(vec![(alias, self.name.clone())].into_iter().collect())
    }
}
