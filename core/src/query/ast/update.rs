use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{ColumnIdent, Expr, FromPair, Location, QueryPair};
use crate::query::grammar::Rule;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ValueItem {
    Default,
    Expr(Expr),
}

impl FromPair for ValueItem {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);
        match pair.as_rule() {
            Rule::value_item => {
                let inner = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("value_item")))?;

                match inner.as_rule() {
                    Rule::keyword_default => Ok(ValueItem::Default),
                    Rule::expr => Expr::from_pair(inner).map(ValueItem::Expr),
                    _ => Err(location.error(SyntaxError::UnexpectedPair("value_item"))),
                }
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("value_item"))),
        }
    }
}

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
                        value: 1,
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
                        value: 1,
                        location,
                    }))),
                ),
            ],
            location,
        },
        Rule::set_clause,
    );
}
