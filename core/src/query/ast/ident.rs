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
    use crate::pest::Parser;
    use crate::query::grammar::Grammar;

    let result_1 = Grammar::parse(Rule::column_ident, "*")
        .unwrap()
        .next()
        .unwrap();
    let result_2 = Grammar::parse(Rule::column_ident, "a.b.*")
        .unwrap()
        .next()
        .unwrap();

    assert_eq!(
        ColumnIdent::from_pair(result_1).unwrap().segments,
        vec!["*".to_string()]
    );
    assert_eq!(
        ColumnIdent::from_pair(result_2).unwrap().segments,
        vec!["a".to_string(), "b".to_string(), "*".to_string()]
    )
}
