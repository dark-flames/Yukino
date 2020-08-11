use crate::mapping::DatabaseValue;
use crate::query::expr::ExpressionStructure;
use syn::export::fmt::Display;

pub enum SelectItem {
    All,
    Item(ExpressionStructure),
    AliasItem {
        expr: ExpressionStructure,
        alias: String,
    },
}

pub struct AssignmentItem {
    pub column_name: String,
    pub value: DatabaseValue,
}

impl AssignmentItem {
    pub fn new<D: Display + ?Sized>(column_name: &D, value: &DatabaseValue) -> AssignmentItem {
        AssignmentItem {
            column_name: column_name.to_string(),
            value: value.clone(),
        }
    }
}

pub enum Order {
    ASC,
    DESC,
}

pub struct OrderByItem {
    pub expr: ExpressionStructure,
    pub order: Order,
}

pub enum JoinType {
    LeftJoin,
    InnerJoin,
    RightJoin,
}

pub struct JoinClause {
    pub alias: String,
    pub condition: ExpressionStructure,
}

#[allow(dead_code)]
pub struct GroupByClause {
    items: Vec<ExpressionStructure>,
    filter: Vec<ExpressionStructure>,
}
