use crate::query::ast::error::SyntaxErrorWithPos;
use crate::query::ast::Location;
use crate::query::grammar::Rule;
use pest::iterators::Pair;

pub type QueryPair<'a> = Pair<'a, Rule>;

pub trait FromPair<Into = Self>
where
    Self: Sized,
{
    fn from_pair(pair: QueryPair) -> Result<Into, SyntaxErrorWithPos>;
}

pub trait Locatable {
    fn location(&self) -> Location;
}
