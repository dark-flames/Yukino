use crate::query::parse::lex::Token;
use crate::query::parse::{Error, Lexer, ParseError};
use std::cell::Cell;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

fn tokens_to_string(tokens: &[Token], with_whitespace: bool) -> String {
    tokens
        .iter()
        .map(|token| token.to_string())
        .fold(String::new(), |mut carry, result| {
            carry.push_str(&result);
            if with_whitespace {
                carry.push(' ');
            }

            carry
        })
}

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
    pub fn get_tokens(&self) -> &[Token] {
        self.tokens.split_at(self.cursor.get()).1
    }

    pub fn peek<E: Parse>(&self) -> bool {
        E::peek(self)
    }

    pub fn parse<E: Parse>(&mut self) -> Result<E, Error> {
        E::parse(self)
    }

    pub fn pop_tokens(&mut self, len: usize) -> Result<Vec<Token>, Error> {
        if len > self.tokens.len() {
            Err(self.error_head(ParseError::UnexpectTokenOffset(self.tokens.len(), len)))
        } else {
            let result = self.get_tokens().split_at(len).0;

            self.cursor.set(self.cursor.get() + len);

            Ok(result.to_vec())
        }
    }

    pub fn pop_token(&mut self) -> Result<Token, Error> {
        self.pop_tokens(1).map(|list| list.first().unwrap().clone())
    }

    pub fn is_empty(&self) -> bool {
        self.cursor.get() == self.tokens.len()
    }

    pub fn error_at<T: Display>(&self, message: T, cursor: usize) -> Error {
        let mut content_cursor = cursor;

        content_cursor = if content_cursor < 2 {
            0
        } else {
            content_cursor - 2
        };

        let tokens = self.tokens.split_at(content_cursor).1;

        let pre = tokens.split_at(cursor - content_cursor).0;

        let content = tokens_to_string(tokens, false);
        let pos = tokens_to_string(pre, false).len();

        Error::new(message, content, pos)
    }

    pub fn error_head<T: Display>(&self, message: T) -> Error {
        self.error_at(message, self.cursor.get())
    }

    pub fn error_offset<T: Display>(&self, message: T, offset: isize) -> Error {
        self.error_at(message, ((self.cursor.get() as isize) + offset) as usize)
    }

    pub fn cursor(&self) -> usize {
        self.cursor.get()
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
        tokens_to_string(&self.tokens, true).fmt(f)
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
