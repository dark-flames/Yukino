use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{Expr, FromPair, Locatable, Location, QueryPair};
use crate::query::grammar::Rule;

#[derive(Debug, Clone)]
pub struct TableReference {
    pub name: String,
    pub alias: Option<String>,
    pub location: Location,
}

impl PartialEq for TableReference {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.alias == other.alias
    }
}

impl Eq for TableReference {}

impl Locatable for TableReference {
    fn location(&self) -> Location {
        self.location
    }
}

impl FromPair for TableReference {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);
        match pair.as_rule() {
            Rule::table_reference => {
                let mut inner = pair.into_inner();

                let parse_ident = |inner_pair: QueryPair| {
                    let inner_location = Location::from(&inner_pair);
                    match inner_pair.as_rule() {
                        Rule::ident | Rule::any_ident => Ok(inner_pair.as_str().to_string()),
                        _ => Err(inner_location.error(SyntaxError::UnexpectedPair("ident"))),
                    }
                };

                let name = inner.next().map(parse_ident).ok_or_else(|| {
                    location.error(SyntaxError::UnexpectedPair("table_reference"))
                })??;

                let alias = inner
                    .next()
                    .map(parse_ident)
                    .map_or(Ok(None), |v| v.map(Some))?;

                Ok(TableReference {
                    name,
                    alias,
                    location,
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("table_reference"))),
        }
    }
}

#[test]
fn test_table_ref() {
    use crate::query::ast::helper::assert_parse_result;

    let location = Location::pos(1);

    assert_parse_result(
        "Test As \"where\"",
        TableReference {
            name: "Test".to_string(),
            alias: Some("where".to_string()),
            location,
        },
        Rule::table_reference,
    );

    assert_parse_result(
        "Test",
        TableReference {
            name: "Test".to_string(),
            alias: None,
            location,
        },
        Rule::table_reference,
    );

    assert_parse_result(
        "Test t",
        TableReference {
            name: "Test".to_string(),
            alias: Some("t".to_string()),
            location,
        },
        Rule::table_reference,
    );

    assert_parse_result(
        "Test AS t",
        TableReference {
            name: "Test".to_string(),
            alias: Some("t".to_string()),
            location,
        },
        Rule::table_reference,
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum JoinType {
    Left,
    Right,
    Full,
    LeftOuter,
    RightOuter,
    FullOuter,
    Inner,
}

impl FromPair for JoinType {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::join_type => {
                let mut inner = pair.into_inner();

                let first = inner
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("join_type")))?;

                let next = inner.next().map(|next_pair| next_pair.as_rule());

                match first.as_rule() {
                    Rule::keyword_inner => {
                        if let Some(Rule::keyword_outer) = next {
                            Err(Location::from(&first)
                                .error(SyntaxError::UnexpectedPair("join_type")))
                        } else {
                            Ok(JoinType::Inner)
                        }
                    }
                    Rule::keyword_left => {
                        if let Some(Rule::keyword_outer) = next {
                            Ok(JoinType::LeftOuter)
                        } else {
                            Ok(JoinType::Left)
                        }
                    }
                    Rule::keyword_right => {
                        if let Some(Rule::keyword_outer) = next {
                            Ok(JoinType::RightOuter)
                        } else {
                            Ok(JoinType::Right)
                        }
                    }
                    Rule::keyword_full => {
                        if let Some(Rule::keyword_outer) = next {
                            Ok(JoinType::FullOuter)
                        } else {
                            Ok(JoinType::Full)
                        }
                    }
                    _ => {
                        Err(Location::from(&first).error(SyntaxError::UnexpectedPair("join_type")))
                    }
                }
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("join_type"))),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum JoinClause {
    NaturalJoin(NaturalJoin),
    CrossJoin(CrossJoin),
    JoinOn(JoinOn),
}

#[derive(Clone, Debug)]
pub struct NaturalJoin {
    pub ty: JoinType,
    pub table: TableReference,
    pub location: Location,
}

impl PartialEq for NaturalJoin {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.table == other.table
    }
}

impl Eq for NaturalJoin {}

#[derive(Clone, Debug)]
pub struct CrossJoin {
    pub table: TableReference,
    pub location: Location,
}

impl PartialEq for CrossJoin {
    fn eq(&self, other: &Self) -> bool {
        self.table == other.table
    }
}

impl Eq for CrossJoin {}

#[derive(Clone, Debug)]
pub struct JoinOn {
    pub ty: JoinType,
    pub table: TableReference,
    pub on: Expr,
    pub location: Location,
}

impl FromPair for JoinClause {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);
        match pair.as_rule() {
            Rule::join_clause => {
                let inner_pair = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("join_clause")))?;

                match inner_pair.as_rule() {
                    Rule::natural_join => {
                        let mut inner = inner_pair.into_inner();

                        Ok(JoinClause::NaturalJoin(NaturalJoin {
                            ty: JoinType::from_pair(inner.next().ok_or_else(|| {
                                location.error(SyntaxError::UnexpectedPair("natural_join"))
                            })?)?,
                            table: TableReference::from_pair(inner.next().ok_or_else(|| {
                                location.error(SyntaxError::UnexpectedPair("natural_join"))
                            })?)?,
                            location,
                        }))
                    }
                    Rule::cross_join => {
                        let mut inner = inner_pair.into_inner();

                        Ok(JoinClause::CrossJoin(CrossJoin {
                            table: TableReference::from_pair(inner.next().ok_or_else(|| {
                                location.error(SyntaxError::UnexpectedPair("natural_join"))
                            })?)?,
                            location,
                        }))
                    }
                    Rule::join_on => {
                        let mut inner = inner_pair.into_inner();

                        Ok(JoinClause::JoinOn(JoinOn {
                            ty: JoinType::from_pair(inner.next().ok_or_else(|| {
                                location.error(SyntaxError::UnexpectedPair("natural_join"))
                            })?)?,
                            table: TableReference::from_pair(inner.next().ok_or_else(|| {
                                location.error(SyntaxError::UnexpectedPair("natural_join"))
                            })?)?,
                            on: Expr::from_pair(inner.next().ok_or_else(|| {
                                location.error(SyntaxError::UnexpectedPair("natural_join"))
                            })?)?,
                            location,
                        }))
                    }
                    _ => Err(location.error(SyntaxError::UnexpectedPair("join_clause"))),
                }
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("join_clause"))),
        }
    }
}

impl Locatable for JoinClause {
    fn location(&self) -> Location {
        match self {
            JoinClause::NaturalJoin(v) => v.location,
            JoinClause::CrossJoin(v) => v.location,
            JoinClause::JoinOn(v) => v.location,
        }
    }
}

impl PartialEq for JoinOn {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.table == other.table && self.on == other.on
    }
}

impl Eq for JoinOn {}

#[test]
fn test_join_clause() {
    use crate::query::ast::expr::{Binary, BinaryOperator};
    use crate::query::ast::helper::assert_parse_result;
    use crate::query::ast::ident::ColumnIdent;
    use crate::query::ast::literal::{Integer, Literal};

    let location = Location::pos(0);

    assert_parse_result(
        "NATURAL INNER JOIN test AS t",
        JoinClause::NaturalJoin(NaturalJoin {
            ty: JoinType::Inner,
            table: TableReference {
                name: "test".to_string(),
                alias: Some("t".to_string()),
                location,
            },
            location,
        }),
        Rule::join_clause,
    );

    assert_parse_result(
        "CROSS JOIN test AS t",
        JoinClause::CrossJoin(CrossJoin {
            table: TableReference {
                name: "test".to_string(),
                alias: Some("t".to_string()),
                location,
            },
            location,
        }),
        Rule::join_clause,
    );

    assert_parse_result(
        "CROSS JOIN test AS t",
        JoinClause::CrossJoin(CrossJoin {
            table: TableReference {
                name: "test".to_string(),
                alias: Some("t".to_string()),
                location,
            },
            location,
        }),
        Rule::join_clause,
    );

    assert_parse_result(
        "INNER JOIN test AS t ON t.id >= 100",
        JoinClause::JoinOn(JoinOn {
            ty: JoinType::Inner,
            table: TableReference {
                name: "test".to_string(),
                alias: Some("t".to_string()),
                location,
            },
            on: Expr::Binary(Binary {
                operator: BinaryOperator::Bte,
                left: Box::new(Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["t".to_string(), "id".to_string()],
                    location,
                })),
                right: Box::new(Expr::Literal(Literal::Integer(Integer {
                    value: "100".to_string(),
                    location,
                }))),
                location,
            }),
            location,
        }),
        Rule::join_clause,
    );

    assert_parse_result(
        "LEFT OUTER JOIN test2 AS t2 ON t2.assoc = t1.id",
        JoinClause::JoinOn(JoinOn {
            ty: JoinType::LeftOuter,
            table: TableReference {
                name: "test2".to_string(),
                alias: Some("t2".to_string()),
                location,
            },
            on: Expr::Binary(Binary {
                operator: BinaryOperator::Eq,
                left: Box::new(Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["t2".to_string(), "assoc".to_string()],
                    location,
                })),
                right: Box::new(Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["t1".to_string(), "id".to_string()],
                    location,
                })),
                location,
            }),
            location,
        }),
        Rule::join_clause,
    );
}

#[derive(Clone, Debug)]
pub struct FromClause {
    pub table: TableReference,
    pub join: Vec<JoinClause>,
    pub location: Location,
}

impl FromPair for FromClause {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::from_clause => {
                let mut inner = pair.into_inner();

                Ok(FromClause {
                    table: TableReference::from_pair(inner.next().ok_or_else(|| {
                        location.error(SyntaxError::UnexpectedPair("from_clause"))
                    })?)?,
                    join: inner.map(JoinClause::from_pair).collect::<Result<_, _>>()?,
                    location,
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("from_clause"))),
        }
    }
}

impl Locatable for FromClause {
    fn location(&self) -> Location {
        self.location
    }
}

impl PartialEq for FromClause {
    fn eq(&self, other: &Self) -> bool {
        self.table == other.table && self.join == other.join
    }
}

impl Eq for FromClause {}

#[test]
fn test_from_clause() {
    use crate::query::ast::expr::{Binary, BinaryOperator};
    use crate::query::ast::helper::assert_parse_result;
    use crate::query::ast::ident::ColumnIdent;

    let location = Location::pos(0);

    assert_parse_result(
        "From test t",
        FromClause {
            table: TableReference {
                name: "test".to_string(),
                alias: Some("t".to_string()),
                location,
            },
            join: vec![],
            location,
        },
        Rule::from_clause,
    );

    assert_parse_result(
        "From test AS t1 INNER JOIN test2 AS t2 ON t2.assoc = t1.id",
        FromClause {
            table: TableReference {
                name: "test".to_string(),
                alias: Some("t1".to_string()),
                location,
            },
            join: vec![JoinClause::JoinOn(JoinOn {
                ty: JoinType::Inner,
                table: TableReference {
                    name: "test2".to_string(),
                    alias: Some("t2".to_string()),
                    location,
                },
                on: Expr::Binary(Binary {
                    operator: BinaryOperator::Eq,
                    left: Box::new(Expr::ColumnIdent(ColumnIdent {
                        segments: vec!["t2".to_string(), "assoc".to_string()],
                        location,
                    })),
                    right: Box::new(Expr::ColumnIdent(ColumnIdent {
                        segments: vec!["t1".to_string(), "id".to_string()],
                        location,
                    })),
                    location,
                }),
                location,
            })],
            location,
        },
        Rule::from_clause,
    );

    assert_parse_result(
        "From test t1 RIGHT JOIN test2 t2 ON t2.assoc = t1. id NATURAL RIGHT OUTER JOIN test2",
        FromClause {
            table: TableReference {
                name: "test".to_string(),
                alias: Some("t1".to_string()),
                location,
            },
            join: vec![
                JoinClause::JoinOn(JoinOn {
                    ty: JoinType::Right,
                    table: TableReference {
                        name: "test2".to_string(),
                        alias: Some("t2".to_string()),
                        location,
                    },
                    on: Expr::Binary(Binary {
                        operator: BinaryOperator::Eq,
                        left: Box::new(Expr::ColumnIdent(ColumnIdent {
                            segments: vec!["t2".to_string(), "assoc".to_string()],
                            location,
                        })),
                        right: Box::new(Expr::ColumnIdent(ColumnIdent {
                            segments: vec!["t1".to_string(), "id".to_string()],
                            location,
                        })),
                        location,
                    }),
                    location,
                }),
                JoinClause::NaturalJoin(NaturalJoin {
                    ty: JoinType::RightOuter,
                    table: TableReference {
                        name: "test2".to_string(),
                        alias: None,
                        location,
                    },
                    location,
                }),
            ],
            location,
        },
        Rule::from_clause,
    )
}

#[derive(Clone, Debug)]
pub struct GroupByClause {
    pub by: Expr,
    pub having: Option<Expr>,
    pub location: Location,
}

impl FromPair for GroupByClause {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::group_by_clause => {
                let mut inner = pair.into_inner();

                Ok(GroupByClause {
                    by: Expr::from_pair(inner.next().ok_or_else(|| {
                        location.error(SyntaxError::UnexpectedPair("group_clause"))
                    })?)?,
                    having: inner
                        .next()
                        .map(Expr::from_pair)
                        .map_or(Ok(None), |v| v.map(Some))?,
                    location,
                })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("group_clause"))),
        }
    }
}

impl Locatable for GroupByClause {
    fn location(&self) -> Location {
        self.location
    }
}

impl PartialEq for GroupByClause {
    fn eq(&self, other: &Self) -> bool {
        self.by == other.by && self.having == other.having
    }
}

impl Eq for GroupByClause {}

#[test]
fn test_group_by() {
    use crate::query::ast::helper::assert_parse_result;
    use crate::query::ast::{Binary, BinaryOperator, ColumnIdent, FunctionCall, Integer, Literal};

    let location = Location::pos(0);

    assert_parse_result(
        "GROUP BY t.id",
        GroupByClause {
            by: Expr::ColumnIdent(ColumnIdent {
                segments: vec!["t".to_string(), "id".to_string()],
                location,
            }),
            having: None,
            location,
        },
        Rule::group_by_clause,
    );

    assert_parse_result(
        "GROUP BY t.assoc Having Sum(t.count) >= 100",
        GroupByClause {
            by: Expr::ColumnIdent(ColumnIdent {
                segments: vec!["t".to_string(), "assoc".to_string()],
                location,
            }),
            having: Some(Expr::Binary(Binary {
                operator: BinaryOperator::Bte,
                left: Box::new(Expr::FunctionCall(FunctionCall {
                    ident: "Sum".to_string(),
                    parameters: vec![Expr::ColumnIdent(ColumnIdent {
                        segments: vec!["t".to_string(), "count".to_string()],
                        location,
                    })],
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
        Rule::group_by_clause,
    )
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Order {
    Desc,
    Asc,
}

impl FromPair for Order {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::order => {
                match pair
                    .into_inner()
                    .next()
                    .map(|inner_pair| inner_pair.as_rule())
                {
                    Some(Rule::order_asc) => Ok(Order::Asc),
                    Some(Rule::order_desc) => Ok(Order::Desc),
                    _ => Err(location.error(SyntaxError::UnexpectedPair("order"))),
                }
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("order"))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrderByClause {
    pub items: Vec<(Expr, Order)>,
    pub location: Location,
}

impl FromPair for OrderByClause {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::order_by_clause => {
                let mut inner = pair.into_inner();

                let mut items = vec![];
                while let Some(inner_pair) = inner.next() {
                    if let Rule::expr = inner_pair.as_rule() {
                        let expr = Expr::from_pair(inner_pair)?;

                        let order = inner.next().map(Order::from_pair).ok_or_else(|| {
                            location.error(SyntaxError::UnexpectedPair("order"))
                        })??;

                        items.push((expr, order));
                    } else {
                        break;
                    }
                }

                Ok(OrderByClause { items, location })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("order_by_clause"))),
        }
    }
}

impl Locatable for OrderByClause {
    fn location(&self) -> Location {
        self.location
    }
}

impl PartialEq for OrderByClause {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Eq for OrderByClause {}

#[test]
fn test_order_by() {
    use crate::query::ast::helper::assert_parse_result;
    use crate::query::ast::ColumnIdent;

    let location = Location::pos(0);

    assert_parse_result(
        "ORDER BY t.assoc DESC",
        OrderByClause {
            items: vec![(
                Expr::ColumnIdent(ColumnIdent {
                    segments: vec!["t".to_string(), "assoc".to_string()],
                    location,
                }),
                Order::Desc,
            )],
            location,
        },
        Rule::order_by_clause,
    )
}

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
