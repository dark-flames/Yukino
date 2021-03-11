use crate::query::ast::error::SyntaxErrorWithPos;
use crate::query::ast::Location;
use crate::query::grammar::Rule;
use pest::iterators::Pair;

pub type QueryPair<'a> = Pair<'a, Rule>;

pub trait Node
where
    Self: Sized,
{
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos>;

    fn location(&self) -> Location;
}
