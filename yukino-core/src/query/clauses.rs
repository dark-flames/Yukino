use crate::mapping::DatabaseValue;

#[macro_export]
macro_rules! alias {
    ($($path: ident).+) => {
        $crate::query::AliasItem {
            path: vec![$(stringify!($path).to_string())*],
            alias: None
        }
    };
    ($($path: ident).+ as $alias: ident) => {
        $crate::query::AliasItem {
            path: vec![$(stringify!($path).to_string()),*],
            alias: Some(stringify!($alias).to_string())
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct AliasItem {
    pub path: Vec<String>,
    pub alias: Option<String>,
}

pub struct AssignmentItem {
    pub column_name: String,
    pub value: DatabaseValue,
}