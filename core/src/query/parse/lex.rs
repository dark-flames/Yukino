use std::cell::Cell;
use regex::{Regex, Captures};
use crate::query::parse::parser::ParseBuffer;
use crate::query::parse::error::{ParseError, Error};

pub enum Token {
    Symbol(SymbolToken)
}

pub trait ReadableToken {
    fn parse(&self, buffer: &LexBuffer) -> Option<Result<Token, ParseError>>;
}

pub enum SymbolToken {
    Add
}

impl ReadableToken for SymbolToken {
    fn parse(&self, buffer: &LexBuffer) -> Option<Result<Token, ParseError>> {
        let pattern = vec![
            (SymbolToken::Add, r"^\S*\+")
        ];
        for (token, regex) in pattern {
            if buffer.parse(Regex::new(regex).unwrap()).is_some() {
                return Some(Ok(Token::Symbol(token)))
            }
        }

        None
    }
}

pub struct LexBuffer<'a> {
    content: &'a str,
    cursor: Cell<usize>
}

impl<'a> LexBuffer<'a> {
    fn rest(&self) -> &str {
        self.content.split_at(self.cursor.get()).1
    }

    fn handle_captures(&self, captures: &Captures) {
        let mut cursor = self.cursor.get();

        cursor += captures.get(0).unwrap().as_str().len();

        self.cursor.set(cursor);
    }

    pub fn trim(&self) {
        let whitespace_regex = Regex::new(r"^\S*").unwrap();

        if let Some(whitespace_result) = whitespace_regex.captures(self.rest()) {
            self.handle_captures(&whitespace_result)
        }
    }

    pub fn parse(&self, regex: Regex) -> Option<Captures> {
        let result = regex.captures(
            self.rest()
        );

        if let Some(captures) = &result {
            self.handle_captures(captures);

            self.trim()
        }

        result
    }

    pub fn error_head(&self, error: ParseError) -> Error {
        Error::head(
            self.rest(),
            error
        )
    }

    pub fn end(&self) -> bool {
        self.rest().is_empty()
    }
}

pub struct Lexer<'a> {
    buffer: LexBuffer<'a>,
    seeds: Vec<Box<dyn ReadableToken>>
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        let buffer = LexBuffer {
            content,
            cursor: Cell::new(0)
        };
        buffer.trim();
        Lexer {
            buffer,
            seeds: vec![]
        }
    }

    pub fn push_seed(&mut self, seed: impl ReadableToken + 'static) -> &mut Self {
        self.seeds.push(Box::new(seed));
        self
    }

    pub fn exec(self) -> Result<ParseBuffer, Error> {
        let mut tokens = vec![];

        while !self.buffer.end() {
            let mut matched = false;
            for seed in self.seeds.iter() {
                if let Some(result) = seed.parse(&self.buffer) {
                    tokens.push(result.map_err(
                        |e| self.buffer.error_head(e)
                    )?);
                    matched = true;
                    break;
                }
            }

            if !matched {
                return Err(self.buffer.error_head(ParseError::UnknownToken))
            }
        }

        Ok(ParseBuffer::new(tokens))
    }
}

