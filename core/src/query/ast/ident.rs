use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::traits::{FromPair, Locatable, QueryPair};
use crate::query::ast::Location;
use crate::query::grammar::Rule;

#[derive(Debug, Clone)]
pub struct ColumnIdent {
    pub segments: Vec<String>,
    pub location: Location,
}

impl PartialEq for ColumnIdent {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

impl Eq for ColumnIdent {}

impl FromPair for ColumnIdent {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::column_ident => {
                let mut segments = vec![];

                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::any => segments.push("*".to_string()),
                        Rule::ident => segments.push(inner_pair.as_str().to_string()),
                        Rule::any_ident => segments.push(inner_pair.as_str().to_string()),
                        _ => {
                            return Err(location.error(SyntaxError::UnexpectedPair("any or ident")))
                        }
                    }
                }

                Ok(ColumnIdent { segments, location })
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("column_ident"))),
        }
    }
}

impl Locatable for ColumnIdent {
    fn location(&self) -> Location {
        self.location
    }
}

#[test]
fn test_ident() {
    use crate::query::ast::helper::assert_parse_result;

    let location = Location::pos(1);

    assert_parse_result(
        "*",
        ColumnIdent {
            segments: vec!["*".to_string()],
            location,
        },
        Rule::column_ident,
    );

    assert_parse_result(
        "a.b2.*",
        ColumnIdent {
            segments: vec!["a".to_string(), "b2".to_string(), "*".to_string()],
            location,
        },
        Rule::column_ident,
    );

    assert_parse_result(
        "t2.assoc",
        ColumnIdent {
            segments: vec!["t2".to_string(), "assoc".to_string()],
            location,
        },
        Rule::column_ident,
    );
}
