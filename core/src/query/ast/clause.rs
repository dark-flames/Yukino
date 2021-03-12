use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::expr::Expr;
use crate::query::ast::traits::{FromPair, Locatable, QueryPair};
use crate::query::ast::Location;
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
    fn assert_table_ref(input: &'static str, result: TableReference) {
        use crate::pest::Parser;
        use crate::query::grammar::Grammar;

        let pair = Grammar::parse(Rule::table_reference, input)
            .unwrap()
            .next()
            .unwrap();

        assert_eq!(TableReference::from_pair(pair).unwrap(), result);
    }
    let location = Location::pos(0);

    assert_table_ref(
        "Test As \"where\"",
        TableReference {
            name: "Test".to_string(),
            alias: Some("where".to_string()),
            location,
        },
    );

    assert_table_ref(
        "Test",
        TableReference {
            name: "Test".to_string(),
            alias: None,
            location,
        },
    );

    assert_table_ref(
        "Test t",
        TableReference {
            name: "Test".to_string(),
            alias: Some("t".to_string()),
            location,
        },
    );
}

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

pub enum JoinClause {
    NaturalJoin(NaturalJoin),
    CrossJoin(CrossJoin),
    JoinOn(JoinOn),
}

pub struct NaturalJoin {
    pub ty: JoinType,
    pub table: TableReference,
    pub location: Location,
}

pub struct CrossJoin {
    pub table: TableReference,
    pub location: Location,
}

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
