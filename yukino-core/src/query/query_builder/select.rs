use crate::mapping::DefinitionManager;
use crate::query::expr::Expression;
use crate::query::{GroupByClause, JoinClause, OrderByItem, QueryError, SelectItem};
use crate::Entity;
use std::any::type_name;
use std::collections::HashMap;

pub struct SelectQueryBuilder {
    definition_manager: &'static DefinitionManager,
    select_items: Vec<SelectItem>,
    alias: HashMap<String, String>,
    join_clauses: Vec<JoinClause>,
    where_clauses: Vec<Expression>,
    group_by_clauses: Option<GroupByClause>,
    order_by_items: Vec<OrderByItem>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl SelectQueryBuilder {
    pub fn resolve_entity_alias<T: Entity>(
        &mut self,
        alias: Option<String>,
    ) -> Result<&mut Self, QueryError> {
        let definitions = self
            .definition_manager
            .get_entity_definitions::<T>()
            .ok_or_else(|| QueryError::UnknownEntity(type_name::<T>()))?;

        for definition in definitions {
            let mut table_alias = String::new();
            let mut char_iter = definition.name.chars();
            let ok = loop {
                if let Some(char) = char_iter.next() {
                    if char == '_' {
                        continue;
                    };

                    table_alias.push(char);

                    if !self.alias.contains_key(&table_alias) {
                        break true;
                    };
                } else {
                    break false;
                }
            };

            if !ok {
                return Err(QueryError::ConflictAlias(definition.name.clone()));
            }

            let result = match &alias {
                Some(entity_alias) => format!("{}_{}", entity_alias, table_alias),
                None => table_alias,
            };

            self.alias.insert(result, definition.name.clone());
        }

        Ok(self)
    }

    pub fn create(definition_manager: &'static DefinitionManager) -> Self {
        SelectQueryBuilder {
            definition_manager,
            select_items: vec![],
            alias: Default::default(),
            join_clauses: vec![],
            where_clauses: vec![],
            group_by_clauses: None,
            order_by_items: vec![],
            limit: None,
            offset: None,
        }
    }

    pub fn select(&mut self, mut items: Vec<SelectItem>) -> &mut Self {
        self.select_items.append(&mut items);

        self
    }

    pub fn from<T: Entity>(&mut self, alias: Option<String>) -> Result<&mut Self, QueryError> {
        self.resolve_entity_alias::<T>(alias)?;

        Ok(self)
    }

    pub fn join<T: Entity>(&mut self, clause: JoinClause) -> Result<&mut Self, QueryError> {
        self.resolve_entity_alias::<T>(Some(clause.alias.clone()))?;
        self.join_clauses.push(clause);

        Ok(self)
    }

    pub fn where_and(&mut self, mut exprs: Vec<Expression>) -> &mut Self {
        self.where_clauses.append(&mut exprs);

        self
    }

    pub fn group(&mut self, clause: GroupByClause) -> &mut Self {
        self.group_by_clauses = Some(clause);

        self
    }

    pub fn order_by(&mut self, mut items: Vec<OrderByItem>) -> &mut Self {
        self.order_by_items.append(&mut items);

        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);

        self
    }
}
