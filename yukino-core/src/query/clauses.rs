use crate::mapping::DatabaseValue;
use crate::query::expr::Expression;
use syn::export::fmt::Display;

pub enum SelectItem {
    All,
    Item(Expression),
    AliasItem { expr: Expression, alias: String },
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
    pub expr: Expression,
    pub order: Order,
}

pub enum JoinType {
    LeftJoin,
    InnerJoin,
    RightJoin,
}

pub struct JoinItem {
    pub table: String,
    pub alias: String,
    pub condition: Expression,
}
