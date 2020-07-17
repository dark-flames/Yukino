use crate::mapping::definition::DefinitionManager;
use crate::query::error::QueryError;
use crate::query::expr::Expression;
use crate::query::{AssignmentItem, JoinItem, OrderByItem, SelectItem};
use crate::Entity;
use std::any::type_name;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct SelectQueryBuilderInitializer {
    select_items: Vec<SelectItem>,
    definition_manager: &'static DefinitionManager,
}

#[allow(dead_code)]
impl SelectQueryBuilderInitializer {
    pub fn new(definition_manager: &'static DefinitionManager) -> Self {
        SelectQueryBuilderInitializer {
            select_items: Vec::new(),
            definition_manager,
        }
    }

    pub fn select(&mut self, items: Vec<SelectItem>) -> &mut Self {
        self.select_items.extend(items.into_iter());
        self
    }

    pub fn then(self) -> QueryBuilder {
        let manager = self.definition_manager;
        let ty = QueryType::Select(self.select_items);

        QueryBuilder::from_initializer(ty, manager)
    }
}

#[allow(dead_code)]
pub struct QueryBuilderInitializer(&'static DefinitionManager);

#[allow(dead_code)]
impl QueryBuilderInitializer {
    pub fn create(definition_manager: &'static DefinitionManager) -> Self {
        QueryBuilderInitializer(definition_manager)
    }

    pub fn select(&self, items: Vec<SelectItem>) -> SelectQueryBuilderInitializer {
        let mut result = SelectQueryBuilderInitializer::new(self.0);
        result.select(items);

        result
    }

    // insert
}

#[allow(dead_code)]
pub enum QueryType {
    DELETE,
    Select(Vec<SelectItem>),
    Update(Vec<AssignmentItem>),
}

#[allow(dead_code)]
pub struct QueryBuilder {
    ty: QueryType,
    /// table_name mapped by alias
    alias: HashMap<String, String>,
    where_conditions: Vec<Expression>,
    join_items: Vec<JoinItem>,
    order_by_items: Vec<OrderByItem>,
    definition_manager: &'static DefinitionManager,
}

#[allow(dead_code)]
impl QueryBuilder {
    pub fn from_initializer(ty: QueryType, definition_manager: &'static DefinitionManager) -> Self {
        QueryBuilder {
            ty,
            alias: HashMap::new(),
            where_conditions: vec![],
            join_items: vec![],
            order_by_items: vec![],
            definition_manager,
        }
    }

    fn resolve_entity_alias<T: Entity>(
        &mut self,
        alias: Option<String>,
    ) -> Result<&mut Self, QueryError> {
        let definitions = self
            .definition_manager
            .get_entity_definitions::<T>()
            .ok_or_else(|| QueryError::UnknownEntity(type_name::<T>()))?;

        for definition in definitions {
            if self
                .alias
                .values()
                .any(|table_name| table_name.eq(&definition.name))
            {
                return Err(QueryError::ConflictAlias(definition.name.clone()));
            }

            let mut len = 1;
            let table_alias = loop {
                let (prefix, _) = definition.name.split_at(len);
                len += 1;

                let prefix = prefix.to_string();
                if !self.alias.contains_key(&prefix) {
                    break prefix;
                }
            };
            let result = match &alias {
                Some(entity_alias) => format!("{}_{}", entity_alias, table_alias),
                None => table_alias,
            };

            self.alias.insert(result, definition.name.clone());
        }

        Ok(self)
    }

    pub fn from<T: Entity>(&mut self, alias: Option<String>) -> Result<&mut Self, QueryError> {
        self.resolve_entity_alias::<T>(alias)?;

        Ok(self)
    }

    pub fn and_where(&mut self, condition: Expression) -> &mut Self {
        self.where_conditions.push(condition);

        self
    }

    pub fn join<T: Entity>(&mut self, item: JoinItem) -> Result<&mut Self, QueryError> {
        self.resolve_entity_alias::<T>(Some(item.alias.clone()))?;
        self.join_items.push(item);

        Ok(self)
    }

    pub fn order_by(&mut self, item: OrderByItem) -> Result<&mut Self, QueryError> {
        self.order_by_items.push(item);

        Ok(self)
    }
}
