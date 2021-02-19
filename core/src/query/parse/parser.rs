use crate::query::parse::lex::Token;
use crate::query::parse::{Error, ParseError, Lexer};
use std::cell::Cell;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub struct TokenStream {
    tokens: Vec<Token>,
    cursor: Cell<usize>,
}

#[derive(Clone)]
pub struct ParseBuffer<'a> {
    tokens: &'a [Token],
    cursor: Cell<usize>,
}

impl<'a> ParseBuffer<'a> {
    pub fn get_token(&self) -> &[Token] {
        self.tokens.split_at(self.cursor.get()).1
    }

    pub fn peek<E: Parse>(&self) -> bool {
        E::peek(self)
    }

    pub fn parse<E: Parse>(&mut self) -> Result<E, Error> {
        E::parse(self)
    }

    pub fn pop_token(&mut self, len: usize) -> Result<Vec<Token>, Error> {
        if len > self.tokens.len() {
            Err(self.error(ParseError::UnexpectTokenOffset(self.tokens.len(), len)))
        } else {
            let result = self.get_token().split_at(len).0;

            self.cursor.set(self.cursor.get() + len);

            Ok(result.to_vec())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor.get() == self.tokens.len()
    }

    pub fn error(&self, _error: ParseError) -> Error {
        unimplemented!()
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
            cursor: self.cursor.clone(),
        }
    }

    pub fn peek<E: Parse>(&self) -> bool {
        self.get_buffer().peek::<E>()
    }

    pub fn parse<E: Parse>(&self) -> Result<E, Error> {
        let mut buffer = self.get_buffer();
        let result = buffer.parse::<E>();
        self.cursor.set(buffer.cursor.get());
        result
    }

    pub fn is_empty(&self) -> bool {
        self.cursor.get() == self.tokens.len()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let result = self.tokens.iter().map(|token| token.to_string()).fold(
            String::new(),
            |mut carry, result| {
                carry.push_str(&result);
                carry.push(' ');

                carry
            },
        );

        write!(f, "{}", result)
    }
}

impl FromStr for TokenStream {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Lexer::new(s).exec()
    }
}

pub trait Parse
where
    Self: Sized,
{
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error>;

    fn peek(buffer: &ParseBuffer) -> bool;
}
