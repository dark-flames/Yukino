use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    Expr, FromClause, FromPair, GroupByClause, Locatable, Location, OrderByClause, QueryPair,
};
use crate::query::grammar::Rule;

#[derive(Clone, Debug)]
pub struct SelectClause {
    pub items: Vec<(Expr, Option<String>)>,
    pub location: Location,
}

impl FromPair for SelectClause {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::select_clause => Ok(SelectClause {
                items: pair
                    .into_inner()
                    .map(|item| {
                        let item_location = Location::from(&item);

                        match item.as_rule() {
                            Rule::select_item => {
                                let mut inner = item.into_inner();
                                Ok((
                                    Expr::from_pair(inner.next().ok_or_else(|| {
                                        item_location
                                            .error(SyntaxError::UnexpectedPair("select_item"))
                                    })?)?,
                                    inner
                                        .next()
                                        .map(|inner_pair| {
                                            let inner_location = Location::from(&inner_pair);

                                            match inner_pair.as_rule() {
                                                Rule::ident | Rule::any_ident => {
                                                    Ok(inner_pair.as_str().to_string())
                                                }
                                                _ => Err(inner_location.error(
                                                    SyntaxError::UnexpectedPair("expr_alias"),
                                                )),
                                            }
                                        })
                                        .map_or(Ok(None), |v| v.map(Some))?,
                                ))
                            }
                            _ => {
                                Err(item_location.error(SyntaxError::UnexpectedPair("select_item")))
                            }
                        }
                    })
                    .collect::<Result<_, _>>()?,
                location,
            }),
            _ => Err(location.error(SyntaxError::UnexpectedPair("select_clause"))),
        }
    }
}

impl Locatable for SelectClause {
    fn location(&self) -> Location {
        self.location
    }
}

impl PartialEq for SelectClause {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Eq for SelectClause {}

#[derive(Debug, Clone)]
pub struct SelectQuery {
    pub select_clause: SelectClause,
    pub from: FromClause,
    pub where_clause: Option<Expr>,
    pub group_by_clause: Option<GroupByClause>,
    pub order_by_clause: Option<OrderByClause>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub location: Location,
}

impl FromPair for SelectQuery {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::select_query => {
                let mut inner = pair.into_inner();

                let select_clause =
                    inner.next().map(SelectClause::from_pair).ok_or_else(|| {
                        location.error(SyntaxError::UnexpectedPair("select_clause"))
                    })??;

                let from = inner
                    .next()
                    .map(FromClause::from_pair)
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("from_clause")))??;

                let current = inner.next();

                let (where_clause, current) = match current.as_ref().map(|p| p.as_rule()) {
                    Some(Rule::expr) => (
                        current
                            .map(Expr::from_pair)
                            .map_or(Ok(None), |v| v.map(Some))?,
                        inner.next(),
                    ),
                    _ => (None, current),
                };

                let (group_by_clause, current) = match current.as_ref().map(|p| p.as_rule()) {
                    Some(Rule::group_by_clause) => (
                        current
                            .map(GroupByClause::from_pair)
                            .map_or(Ok(None), |v| v.map(Some))?,
                        inner.next(),
                    ),
                    _ => (None, current),
                };

                let (order_by_clause, current) = match current.as_ref().map(|p| p.as_rule()) {
                    Some(Rule::order_by_clause) => (
                        current
                            .map(OrderByClause::from_pair)
                            .map_or(Ok(None), |v| v.map(Some))?,
                        inner.next(),
                    ),
                    _ => (None, current),
                };

                let (limit, current) = match current.as_ref().map(|p| p.as_rule()) {
                    Some(Rule::limit_clause) => (
                        current
                            .map(|inner_pair| {
                                let inner_location = Location::from(&inner_pair);
                                match inner_pair.as_rule() {
                                    Rule::limit_clause => {
                                        let int_pair = inner_pair.into_inner().next();
                                        match int_pair.as_ref().map(|v| v.as_rule()) {
                                            Some(Rule::int) => {
                                                let content = int_pair.unwrap().as_str();

                                                content.parse().map_err(|_| {
                                                    inner_location.error(
                                                        SyntaxError::CannotParseIntoInteger(
                                                            content.to_string(),
                                                        ),
                                                    )
                                                })
                                            }
                                            _ => Err(inner_location
                                                .error(SyntaxError::UnexpectedPair("int"))),
                                        }
                                    }
                                    _ => Err(inner_location
                                        .error(SyntaxError::UnexpectedPair("limit_clause"))),
                                }
                            })
                            .map_or(Ok(None), |v| v.map(Some))?,
                        inner.next(),
                    ),
                    _ => (None, current),
                };

                let offset =
                    current
                        .map(|inner_pair| {
                            let inner_location = Location::from(&inner_pair);
                            match inner_pair.as_rule() {
                                Rule::offset_clause => {
                                    let int_pair = inner_pair.into_inner().next();
                                    match int_pair.as_ref().map(|v| v.as_rule()) {
                                        Some(Rule::int) => {
                                            let content = int_pair.unwrap().as_str();

                                            content.parse().map_err(|_| {
                                                inner_location.error(
                                                    SyntaxError::CannotParseIntoInteger(
                                                        content.to_string(),
                                                    ),
                                                )
                                            })
                                        }
                                        _ => Err(inner_location
                                            .error(SyntaxError::UnexpectedPair("int"))),
                                    }
                                }
                                _ => Err(inner_location
                                    .error(SyntaxError::UnexpectedPair("offset_clause"))),
                            }
                        })
                        .map_or(Ok(None), |v| v.map(Some))?;

                Ok(SelectQuery {
                    select_clause,
                    from,
                    where_clause,
                    group_by_clause,
                    order_by_clause,
                    limit,
                    offset,
                    location,
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("select_query"))),
        }
    }
}

impl Locatable for SelectQuery {
    fn location(&self) -> Location {
        self.location
    }
}

impl PartialEq for SelectQuery {
    fn eq(&self, other: &Self) -> bool {
        self.select_clause == other.select_clause
            && self.from == other.from
            && self.where_clause == other.where_clause
            && self.group_by_clause == other.group_by_clause
            && self.order_by_clause == other.order_by_clause
            && self.limit == other.limit
            && self.offset == other.offset
    }
}

impl Eq for SelectQuery {}

#[test]
fn test_select() {
    use crate::query::ast::helper::assert_parse_result;
    use crate::query::ast::*;

    let location = Location::pos(0);

    assert_parse_result(
        "Select * from Test AS t WHERE t.id >= 100 GROUP BY t.ty HAVING t.ty != 3 Order By COUNT(t.id) DESC LIMIT 10 OFFSET 1",
        SelectQuery {
            select_clause: SelectClause {
                items: vec![(Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["*".to_string()],
                    location
                }), None)],
                location
            },
            from: FromClause {
                table: TableReference {
                    name: "Test".to_string(),
                    alias: Some("t".to_string()),
                    location
                },
                join: vec![],
                location
            },
            where_clause: Some(Expr::Binary(Binary {
                operator: BinaryOperator::Bte,
                left: Box::new(Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["t".to_string(), "id".to_string()],
                    location
                })),
                right: Box::new(Expr::Literal(Literal::Integer(Integer {
                    value: 100,
                    location
                }))),
                location
            })),
            group_by_clause: Some(GroupByClause {
                by: Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["t".to_string(), "ty".to_string()],
                    location
                }),
                having: Some(Expr::Binary(Binary {
                    operator: BinaryOperator::Neq,
                    left: Box::new(Expr::ColumnIdent(ColumnIdent {
                        segments: vec!["t".to_string(), "ty".to_string()],
                        location
                    })),
                    right: Box::new(Expr::Literal(Literal::Integer(Integer {
                        value: 3,
                        location
                    }))),
                    location
                })),
                location
            }),
            order_by_clause: Some(OrderByClause {
                items: vec![(Expr::FunctionCall(FunctionCall {
                    ident: "COUNT".to_string(),
                    parameters: vec![Expr::ColumnIdent(ColumnIdent {
                        segments: vec!["t".to_string(), "id".to_string()],
                        location
                    })],
                    location
                }), Order::Desc)],
                location
            }),
            limit: Some(10),
            offset: Some(1),
            location
        },
        Rule::select_query
    );

    assert_parse_result(
        "Select * from Test AS t WHERE t.id >= 100 Order By COUNT(t.id) DESC LIMIT 10 OFFSET 1",
        SelectQuery {
            select_clause: SelectClause {
                items: vec![(
                    Expr::ColumnIdent(ColumnIdent {
                        segments: vec!["*".to_string()],
                        location,
                    }),
                    None,
                )],
                location,
            },
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
            group_by_clause: None,
            order_by_clause: Some(OrderByClause {
                items: vec![(
                    Expr::FunctionCall(FunctionCall {
                        ident: "COUNT".to_string(),
                        parameters: vec![Expr::ColumnIdent(ColumnIdent {
                            segments: vec!["t".to_string(), "id".to_string()],
                            location,
                        })],
                        location,
                    }),
                    Order::Desc,
                )],
                location,
            }),
            limit: Some(10),
            offset: Some(1),
            location,
        },
        Rule::select_query,
    );
}
