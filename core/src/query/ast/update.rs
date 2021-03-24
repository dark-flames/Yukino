use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    ColumnIdent, Expr, FromPair, Locatable, Location, QueryPair, TableReference, ValueItem,
};
use crate::query::grammar::Rule;

#[derive(Clone, Debug)]
pub struct SetClause {
    pub items: Vec<(ColumnIdent, ValueItem)>,
    pub location: Location,
}

impl FromPair for SetClause {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::set_clause => {
                let mut inner = pair.into_inner();

                let first = inner
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("set_clause")))?;

                let mut items = vec![];

                match first.as_rule() {
                    Rule::assignments => {
                        let mut assignments_inner = first.into_inner();

                        while let Some(ident_item) = assignments_inner.next() {
                            items.push((
                                ColumnIdent::from_pair(ident_item)?,
                                assignments_inner
                                    .next()
                                    .ok_or_else(|| {
                                        location.error(SyntaxError::UnexpectedPair("assignments"))
                                    })
                                    .map(ValueItem::from_pair)??,
                            ));
                        }
                    }
                    Rule::column_list => {
                        let column_idents = first
                            .into_inner()
                            .map(ColumnIdent::from_pair)
                            .collect::<Result<Vec<_>, _>>()?;

                        let value_items = inner
                            .next()
                            .ok_or_else(|| {
                                location.error(SyntaxError::UnexpectedPair("set_clause"))
                            })?
                            .into_inner()
                            .map(ValueItem::from_pair)
                            .collect::<Result<Vec<_>, _>>()?;

                        items = column_idents
                            .into_iter()
                            .zip(value_items.into_iter())
                            .collect()
                    }
                    _ => return Err(location.error(SyntaxError::UnexpectedPair("set_clause"))),
                };

                Ok(SetClause { items, location })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("set_clause"))),
        }
    }
}

impl PartialEq for SetClause {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Eq for SetClause {}

impl Locatable for SetClause {
    fn location(&self) -> Location {
        self.location
    }
}

#[test]
fn test_set() {
    use crate::query::ast::helper::assert_parse_result;
    use crate::query::ast::*;

    let location = Location::pos(0);

    assert_parse_result(
        "SET a = default, b = 1",
        SetClause {
            items: vec![
                (
                    ColumnIdent {
                        segments: vec!["a".to_string()],
                        location,
                    },
                    ValueItem::Default,
                ),
                (
                    ColumnIdent {
                        segments: vec!["b".to_string()],
                        location,
                    },
                    ValueItem::Expr(Expr::Literal(Literal::Integer(Integer {
                        value: "1".to_string(),
                        location,
                    }))),
                ),
            ],
            location,
        },
        Rule::set_clause,
    );

    assert_parse_result(
        "SET (a, b) = (default, 1)",
        SetClause {
            items: vec![
                (
                    ColumnIdent {
                        segments: vec!["a".to_string()],
                        location,
                    },
                    ValueItem::Default,
                ),
                (
                    ColumnIdent {
                        segments: vec!["b".to_string()],
                        location,
                    },
                    ValueItem::Expr(Expr::Literal(Literal::Integer(Integer {
                        value: "1".to_string(),
                        location,
                    }))),
                ),
            ],
            location,
        },
        Rule::set_clause,
    );
}

#[derive(Clone, Debug)]
pub struct UpdateQuery {
    pub update_table: TableReference,
    pub set_clause: SetClause,
    pub from_table: Option<TableReference>,
    pub where_clause: Option<Expr>,
    pub location: Location,
}

impl FromPair for UpdateQuery {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::update_query => {
                let mut inner = pair.into_inner();

                let update_table =
                    inner
                        .next()
                        .map(TableReference::from_pair)
                        .ok_or_else(|| {
                            location.error(SyntaxError::UnexpectedPair("table_reference"))
                        })??;

                let set_clause = inner
                    .next()
                    .map(SetClause::from_pair)
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("set_clause")))??;

                let current = inner.next();

                let (from_table, current) = match current.as_ref().map(|p| p.as_rule()) {
                    Some(Rule::table_reference) => (
                        current
                            .map(TableReference::from_pair)
                            .map_or(Ok(None), |v| v.map(Some))?,
                        inner.next(),
                    ),
                    _ => (None, current),
                };

                let where_clause = match current.as_ref().map(|p| p.as_rule()) {
                    Some(Rule::expr) => current
                        .map(Expr::from_pair)
                        .map_or(Ok(None), |v| v.map(Some))?,
                    _ => None,
                };

                Ok(UpdateQuery {
                    update_table,
                    set_clause,
                    from_table,
                    where_clause,
                    location,
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("update_query"))),
        }
    }
}

impl PartialEq for UpdateQuery {
    fn eq(&self, other: &Self) -> bool {
        self.update_table == other.update_table
            && self.set_clause == other.set_clause
            && self.from_table == other.from_table
            && self.where_clause == other.where_clause
    }
}

impl Eq for UpdateQuery {}

impl Locatable for UpdateQuery {
    fn location(&self) -> Location {
        self.location
    }
}

#[test]
fn test_update() {
    use crate::query::ast::helper::assert_parse_result;
    use crate::query::ast::*;

    let location = Location::pos(0);

    assert_parse_result(
        "UPDATE TEST1 t SET t.a = default, t.b = 1 FROM TEST2 t2 WHERE t.id > 100",
        UpdateQuery {
            update_table: TableReference {
                name: "TEST1".to_string(),
                alias: Some("t".to_string()),
                location,
            },
            set_clause: SetClause {
                items: vec![
                    (
                        ColumnIdent {
                            segments: vec!["t".to_string(), "a".to_string()],
                            location,
                        },
                        ValueItem::Default,
                    ),
                    (
                        ColumnIdent {
                            segments: vec!["t".to_string(), "b".to_string()],
                            location,
                        },
                        ValueItem::Expr(Expr::Literal(Literal::Integer(Integer {
                            value: "1".to_string(),
                            location,
                        }))),
                    ),
                ],
                location,
            },
            from_table: Some(TableReference {
                name: "TEST2".to_string(),
                alias: Some("t2".to_string()),
                location,
            }),
            where_clause: Some(Expr::Binary(Binary {
                operator: BinaryOperator::Bt,
                left: Box::new(Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["t".to_string(), "id".to_string()],
                    location,
                })),
                right: Box::new(Expr::Literal(Literal::Integer(Integer {
                    value: "100".to_string(),
                    location,
                }))),
                location,
            })),
            location,
        },
        Rule::update_query,
    )
}
