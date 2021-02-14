use crate::query::parse::lex::Token;
use std::cell::Cell;

#[allow(dead_code)]
pub struct ParseBuffer {
    tokens: Vec<Token>,
    cursor: Cell<usize>,
}

impl ParseBuffer {
    pub fn new(tokens: Vec<Token>) -> Self {
        ParseBuffer {
            tokens,
            cursor: Cell::new(0),
        }
    }
}
