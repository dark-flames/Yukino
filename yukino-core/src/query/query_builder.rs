use crate::mapping::definition::DefinitionManager;
use crate::query::error::QueryError;
use crate::query::expr::Expression;
use crate::query::{AssignmentItem, JoinItem, SelectItem};
use crate::Entity;
use std::any::type_name;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct SelectQueryBuilderInitializer {
    items: Vec<SelectItem>,
    definition_manager: &'static DefinitionManager,
}

#[allow(dead_code)]
impl SelectQueryBuilderInitializer {
    pub fn new(definition_manager: &'static DefinitionManager) -> Self {
        SelectQueryBuilderInitializer {
            items: Vec::new(),
            definition_manager,
        }
    }
    pub fn add_select(&mut self, item: SelectItem) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn add_selects(&mut self, items: Vec<SelectItem>) -> &mut Self {
        self.items.extend(items.into_iter());
        self
    }

    pub fn from<T: Entity>(self, alias: Option<&str>) -> Result<QueryBuilder, QueryError> {
        let definition_manager = self.definition_manager;
        let ty = QueryType::Select(self.items);
        QueryBuilder::from_query_type::<T>(ty, alias, definition_manager)
    }
}

#[allow(dead_code)]
pub struct QueryBuilderInitializer(&'static DefinitionManager);

#[allow(dead_code)]
impl QueryBuilderInitializer {
    pub fn create(definition_manager: &'static DefinitionManager) -> Self {
        QueryBuilderInitializer(definition_manager)
    }
    pub fn select(&self, item: SelectItem) -> SelectQueryBuilderInitializer {
        let mut result = SelectQueryBuilderInitializer::new(self.0);
        result.add_select(item);

        result
    }

    pub fn multi_select(&self, items: Vec<SelectItem>) -> SelectQueryBuilderInitializer {
        let mut result = SelectQueryBuilderInitializer::new(self.0);
        result.add_selects(items);

        result
    }

    pub fn delete_from<T: Entity>(&self, alias: Option<&str>) -> Result<QueryBuilder, QueryError> {
        QueryBuilder::from_query_type::<T>(QueryType::DELETE, alias, self.0)
    }

    pub fn update<T: Entity>(
        &self,
        assignments: Vec<AssignmentItem>,
    ) -> Result<QueryBuilder, QueryError> {
        QueryBuilder::from_query_type::<T>(QueryType::Update(assignments), None, self.0)
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
    root_alias: Option<String>,
    root_name: &'static str, // todo: other ident
    where_conditions: Vec<Expression>,
    join_items: Vec<JoinItem>,
    definition_manager: &'static DefinitionManager,
}

#[allow(dead_code)]
impl QueryBuilder {
    pub fn from_query_type<T: Entity>(
        ty: QueryType,
        alias: Option<&str>,
        definition_manager: &'static DefinitionManager,
    ) -> Result<Self, QueryError> {
        let root_name = type_name::<T>();

        let mut result = QueryBuilder {
            ty,
            alias: HashMap::new(),
            root_alias: alias.map(|s| s.to_string()),
            root_name,
            where_conditions: Vec::new(),
            join_items: Vec::new(),
            definition_manager,
        };

        result.resolve_entity_alias::<T>(alias)?;

        Ok(result)
    }

    fn resolve_entity_alias<T: Entity>(
        &mut self,
        alias: Option<&str>,
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
            let result = match alias {
                Some(entity_alias) => format!("{}_{}", entity_alias, table_alias),
                None => table_alias,
            };

            self.alias.insert(result, definition.name.clone());
        }

        Ok(self)
    }

    pub fn and_where(&mut self, condition: Expression) -> &mut Self {
        self.where_conditions.push(condition);

        self
    }
}
