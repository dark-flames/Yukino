use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    DeleteQuery, FromPair, Locatable, Location, QueryPair, SelectQuery, UpdateQuery,
};
use crate::query::grammar::Rule;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Query {
    Delete(Box<DeleteQuery>),
    Select(Box<SelectQuery>),
    Update(Box<UpdateQuery>),
}

impl FromPair for Query {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);
        match pair.as_rule() {
            Rule::query => {
                let inner = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("query")))?;

                match inner.as_rule() {
                    Rule::delete_query => DeleteQuery::from_pair(inner)
                        .map(Box::new)
                        .map(Query::Delete),
                    Rule::select_query => SelectQuery::from_pair(inner)
                        .map(Box::new)
                        .map(Query::Select),
                    Rule::update_query => UpdateQuery::from_pair(inner)
                        .map(Box::new)
                        .map(Query::Update),
                    _ => Err(location.error(SyntaxError::UnexpectedPair("query"))),
                }
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("query"))),
        }
    }
}

impl Locatable for Query {
    fn location(&self) -> Location {
        match self {
            Query::Delete(d) => d.location(),
            Query::Select(s) => s.location(),
            Query::Update(u) => u.location(),
        }
    }
}
