use crate::query::{AliasItem, AssignmentItem};
use crate::Entity;
use std::any::type_name;

#[allow(dead_code)]
pub struct SelectQueryBuilderInitializer(Vec<AliasItem>);

#[allow(dead_code)]
impl SelectQueryBuilderInitializer {
    pub fn new() -> Self {
        SelectQueryBuilderInitializer(Vec::new())
    }
    pub fn add_select(&mut self, item: AliasItem) -> &mut Self {
        self.0.push(item);
        self
    }

    pub fn add_selects(&mut self, items: Vec<AliasItem>) -> &mut Self {
        self.0.extend(items.into_iter());
        self
    }

    pub fn from<T: Entity>(self, alias: Option<String>) -> QueryBuilder {
        let ty = QueryType::Select(self.0);
        QueryBuilder::from_query_type::<T>(ty, alias)
    }
}

#[allow(dead_code)]
pub struct UpdateQueryBuilderInitializer(Vec<AssignmentItem>);

#[allow(dead_code)]
pub struct QueryBuilderInitializer;

#[allow(dead_code)]
impl QueryBuilderInitializer {
    pub fn select(item: AliasItem) -> SelectQueryBuilderInitializer {
        let mut result = SelectQueryBuilderInitializer::new();
        result.add_select(item);

        result
    }

    pub fn multi_select(items: Vec<AliasItem>) -> SelectQueryBuilderInitializer {
        let mut result = SelectQueryBuilderInitializer::new();
        result.add_selects(items);

        result
    }

    pub fn insert_into<T: Entity>() -> QueryBuilder {
        QueryBuilder::from_query_type::<T>(QueryType::Insert, None)
    }

    pub fn delete_from<T: Entity>(alias: Option<String>) -> QueryBuilder {
        QueryBuilder::from_query_type::<T>(QueryType::DELETE, alias)
    }

    pub fn update<T: Entity>(assignments: Vec<AssignmentItem>) -> QueryBuilder {
        QueryBuilder::from_query_type::<T>(QueryType::Update(assignments), None)
    }
}

#[allow(dead_code)]
pub enum QueryType {
    Insert,
    DELETE,
    Select(Vec<AliasItem>),
    Update(Vec<AssignmentItem>),
}

#[allow(dead_code)]
pub struct QueryBuilder {
    ty: QueryType,
    root_alias: Option<String>,
    root_name: String,
}

#[allow(dead_code)]
impl QueryBuilder {
    pub fn create() -> QueryBuilderInitializer {
        QueryBuilderInitializer
    }

    pub fn from_query_type<T: Entity>(ty: QueryType, alias: Option<String>) -> Self {
        QueryBuilder {
            ty,
            root_alias: alias,
            root_name: type_name::<T>().to_string(),
        }
    }
}
