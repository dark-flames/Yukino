use crate::query::parse::lex::Token;
use std::cell::Cell;
use crate::query::parse::ParseError;

pub struct TokenStream {
    tokens: Vec<Token>,
    cursor: Cell<usize>,
}

#[derive(Clone)]
pub struct ParseBuffer<'a> {
    tokens: &'a [Token],
    cursor: Cell<usize>
}

impl<'a> ParseBuffer<'a> {
    pub fn get_token(&self) -> &[Token] {
        self.tokens.split_at(self.cursor.get()).1
    }

    pub fn peek<E: Parse>(&self) -> bool {
        E::peek(self)
    }

    pub fn parse<E: Parse>(&mut self) -> Result<E, ParseError> {
        let result = E::parse(self);

        result
    }

    pub fn pop_token(&mut self, len: usize) -> Result<Vec<Token>, ParseError> {
        if len > self.tokens.len() {
            Err(ParseError::UnexpectTokenOffset(self.tokens.len(), len))
        } else {
            let result = self.get_token().split_at(len).0;

            self.cursor.set(self.cursor.get() + len);

            Ok(result.iter().cloned().collect())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor.get() == self.tokens.len()
    }
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream {
            tokens,
            cursor: Cell::new(0),
        }
    }

    fn get_buffer(&self) -> ParseBuffer {
        ParseBuffer {
            tokens: self.tokens.as_slice(),
            cursor: self.cursor.clone()
        }
    }

    pub fn peek<E: Parse>(&self) -> bool {
        self.get_buffer().peek::<E>()
    }

    pub fn parse<E: Parse>(&self) -> Result<E, ParseError> {
        let mut buffer = self.get_buffer();
        let result = buffer.parse::<E>();
        self.cursor.set(buffer.cursor.get());
        result
    }

    pub fn empty(&self) -> bool {
        self.cursor.get() == self.tokens.len()
    }
}

pub trait Parse where Self: Sized {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, ParseError>;

    fn peek(buffer: &ParseBuffer) -> bool;
}
