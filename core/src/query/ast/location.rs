use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::grammar::Rule;
use pest::error::InputLocation;
use pest::{iterators::Pair, Position, Span};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Copy, Clone)]
pub enum Location {
    Pos(usize),
    Span(usize, usize),
}

impl Location {
    pub fn pos(pos: usize) -> Location {
        Location::Pos(pos)
    }

    pub fn span(start: usize, end: usize) -> Location {
        Location::Span(start, end)
    }

    pub fn error(&self, error: SyntaxError) -> SyntaxErrorWithPos {
        SyntaxErrorWithPos {
            error,
            location: *self,
        }
    }

    pub fn start(&self) -> usize {
        match self {
            Location::Pos(p) => *p,
            Location::Span(s, _) => *s,
        }
    }

    pub fn end(&self) -> usize {
        match self {
            Location::Pos(p) => *p,
            Location::Span(_, e) => *e,
        }
    }
}

impl From<InputLocation> for Location {
    fn from(location: InputLocation) -> Location {
        match location {
            InputLocation::Pos(pos) => Location::Pos(pos),
            InputLocation::Span((start, end)) => Location::Span(start, end),
        }
    }
}

impl<'a> From<Position<'a>> for Location {
    fn from(pos: Position<'a>) -> Location {
        Location::pos(pos.pos())
    }
}

impl<'a> From<Span<'a>> for Location {
    fn from(span: Span<'a>) -> Location {
        Location::Span(span.start(), span.end())
    }
}

impl<'a> From<&Pair<'a, Rule>> for Location {
    fn from(pair: &Pair<'a, Rule>) -> Location {
        Location::from(pair.as_span())
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Location::Pos(pos) => write!(f, "{}", pos),
            Location::Span(s, e) => write!(f, "{}_{}", s, e),
        }
    }
}
