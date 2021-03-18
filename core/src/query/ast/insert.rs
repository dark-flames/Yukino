use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Expr, FromPair, Locatable, Location, QueryPair};
use crate::query::grammar::Rule;

#[derive(Clone, Debug)]
pub struct InsertQuery {
    pub table: String,
    pub columns: Option<Vec<String>>,
    pub values: Vec<Expr>,
    pub location: Location,
}

impl PartialEq for InsertQuery {
    fn eq(&self, other: &Self) -> bool {
        self.table == other.table && self.columns == other.columns && self.values == other.values
    }
}

impl Eq for InsertQuery {}

impl Locatable for InsertQuery {
    fn location(&self) -> Location {
        self.location
    }
}

impl FromPair for InsertQuery {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);
        match pair.as_rule() {
            Rule::insert_query => {
                let mut inner = pair.into_inner();

                let table = inner
                    .next()
                    .map(|inner_pair| {
                        let inner_location = Location::from(&inner_pair);

                        match inner_pair.as_rule() {
                            Rule::ident | Rule::any_ident => Ok(inner_pair.as_str().to_string()),
                            _ => Err(inner_location.error(SyntaxError::UnexpectedPair("ident"))),
                        }
                    })
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("insert_query")))??;

                let (columns, values_pair) = match inner
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("insert_query")))?
                {
                    inner_pair if inner_pair.as_rule() == Rule::column_list => (
                        Some(
                            inner_pair
                                .into_inner()
                                .map(|ident_pair| {
                                    let ident_location = Location::from(&ident_pair);

                                    match ident_pair.as_rule() {
                                        Rule::ident | Rule::any_ident => {
                                            Ok(ident_pair.as_str().to_string())
                                        }
                                        _ => Err(ident_location
                                            .error(SyntaxError::UnexpectedPair("ident"))),
                                    }
                                })
                                .collect::<Result<Vec<_>, _>>()?,
                        ),
                        inner.next().ok_or_else(|| {
                            location.error(SyntaxError::UnexpectedPair("insert_query"))
                        })?,
                    ),
                    others => (None, others),
                };

                let values = values_pair
                    .into_inner()
                    .map(Expr::from_pair)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(InsertQuery {
                    table,
                    columns,
                    values,
                    location,
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("insert_query"))),
        }
    }
}
