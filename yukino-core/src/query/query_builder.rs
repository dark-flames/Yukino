use crate::query::{AssignmentItem, SelectItem, JoinItem};
use crate::Entity;
use std::any::type_name;
use crate::query::expr::Expression;

#[allow(dead_code)]
pub struct SelectQueryBuilderInitializer(Vec<SelectItem>);

#[allow(dead_code)]
impl SelectQueryBuilderInitializer {
    pub fn new() -> Self {
        SelectQueryBuilderInitializer(Vec::new())
    }
    pub fn add_select(&mut self, item: SelectItem) -> &mut Self {
        self.0.push(item);
        self
    }

    pub fn add_selects(&mut self, items: Vec<SelectItem>) -> &mut Self {
        self.0.extend(items.into_iter());
        self
    }

    pub fn from<T: Entity>(self, alias: Option<&str>) -> QueryBuilder {
        let ty = QueryType::Select(self.0);
        QueryBuilder::from_query_type::<T>(ty, alias)
    }
}

#[allow(dead_code)]
pub struct UpdateQueryBuilderInitializer(Vec<AssignmentItem>);

pub struct InsertQueryBuilder(Vec<AssignmentItem>);

#[allow(dead_code)]
pub struct QueryBuilderInitializer;

#[allow(dead_code)]
impl QueryBuilderInitializer {
    pub fn select(item: SelectItem) -> SelectQueryBuilderInitializer {
        let mut result = SelectQueryBuilderInitializer::new();
        result.add_select(item);

        result
    }

    pub fn multi_select(items: Vec<SelectItem>) -> SelectQueryBuilderInitializer {
        let mut result = SelectQueryBuilderInitializer::new();
        result.add_selects(items);

        result
    }

    pub fn insert_into<T: Entity>(assignments: Vec<AssignmentItem>) -> InsertQueryBuilder {
        InsertQueryBuilder(assignments)
    }

    pub fn delete_from<T: Entity>(alias: Option<&str>) -> QueryBuilder {
        QueryBuilder::from_query_type::<T>(QueryType::DELETE, alias)
    }

    pub fn update<T: Entity>(assignments: Vec<AssignmentItem>) -> QueryBuilder {
        QueryBuilder::from_query_type::<T>(QueryType::Update(assignments), None)
    }
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
    root_alias: Option<String>,
    root_name: String, // todo: other ident
    where_conditions: Vec<Expression>,
    join_items: Vec<JoinItem>
}

#[allow(dead_code)]
impl QueryBuilder {
    pub fn create() -> QueryBuilderInitializer {
        QueryBuilderInitializer
    }

    pub fn from_query_type<T: Entity>(ty: QueryType, alias: Option<&str>) -> Self {
        QueryBuilder {
            ty,
            root_alias: alias.map(|s| s.to_string()),
            root_name: type_name::<T>().to_string(),
            where_conditions: Vec::new(),
            join_items: Vec::new()
        }
    }

    pub fn and_where(&mut self, condition: Expression) -> &mut Self {
        self.where_conditions.push(condition);

        self
    }
}
