use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Expr, FromClause, FromPair, Location, QueryPair};
use crate::query::grammar::Rule;

#[derive(Clone, Debug)]
pub struct DeleteQuery {
    pub from: FromClause,
    pub where_clause: Option<Expr>,
    pub location: Location,
}

impl FromPair for DeleteQuery {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::delete_query => {
                let mut inner = pair.into_inner();

                Ok(DeleteQuery {
                    from: inner.next().map(FromClause::from_pair).ok_or_else(|| {
                        location.error(SyntaxError::UnexpectedPair("from_clause"))
                    })??,
                    where_clause: inner
                        .next()
                        .map(Expr::from_pair)
                        .map_or(Ok(None), |v| v.map(Some))?,
                    location,
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("delete_query"))),
        }
    }
}

impl PartialEq for DeleteQuery {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.where_clause == other.where_clause
    }
}

impl Eq for DeleteQuery {}

#[test]
fn test_delete_query() {
    use crate::query::ast::helper::assert_parse_result;
    use crate::query::ast::*;

    let location = Location::pos(0);

    assert_parse_result(
        "DELETE FROM Test AS t WHERE t.id >= 100 ",
        DeleteQuery {
            from: FromClause {
                table: TableReference {
                    name: "Test".to_string(),
                    alias: Some("t".to_string()),
                    location,
                },
                join: vec![],
                location,
            },
            where_clause: Some(Expr::Binary(Binary {
                operator: BinaryOperator::Bte,
                left: Box::new(Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["t".to_string(), "id".to_string()],
                    location,
                })),
                right: Box::new(Expr::Literal(Literal::Integer(Integer {
                    value: 100,
                    location,
                }))),
                location,
            })),
            location,
        },
        Rule::delete_query,
    );
}
