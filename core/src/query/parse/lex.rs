use crate::query::parse::error::{Error, ParseError};
use crate::query::parse::parser::TokenStream;
use regex::{Captures, Regex};
use std::cell::Cell;
use crate::query::parse::Token::Symbol;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone)]
pub enum Token {
    Symbol(SymbolToken),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Symbol(symbol) => symbol.fmt(f)
        }
    }
}

pub trait ReadableToken: Display{
    fn parse(&self, buffer: &LexBuffer) -> Option<Result<Token, ParseError>>;
}

macro_rules! symbol {
    (
        $(($token: tt $name: ident $pattern: expr)),*
    ) => {
        #[derive(Clone)]
        pub enum SymbolToken {
            $($name),*
        }

        impl ReadableToken for SymbolToken {
            fn parse(&self, buffer: &LexBuffer) -> Option<Result<Token, ParseError>> {
                let pattern = vec![
                    $((SymbolToken::$name, $pattern)),*
                ];
                for (token, regex) in pattern {
                    if buffer.parse(Regex::new(regex).unwrap()).is_some() {
                        return Some(Ok(Token::Symbol(token)));
                    }
                }

                None
            }
        }

        impl Display for SymbolToken {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "{}", match self {
                    $(SymbolToken::$name => $token),*
                })
            }
        }
    };
}



symbol! {
    ("+" Add r"^\+"),
    ("*" Mul r"^\*")
}

pub struct LexBuffer<'a> {
    content: &'a str,
    cursor: Cell<usize>,
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
        let whitespace_regex = Regex::new(r"^\s+").unwrap();

        if let Some(whitespace_result) = whitespace_regex.captures(self.rest()) {
            self.handle_captures(&whitespace_result)
        }
    }

    pub fn parse(&self, regex: Regex) -> Option<Captures> {
        let result = regex.captures(self.rest());

        if let Some(captures) = &result {
            self.handle_captures(captures);

            self.trim()
        }

        result
    }

    pub fn error_head(&self, error: ParseError) -> Error {
        Error::head(self.rest(), error)
    }

    pub fn end(&self) -> bool {
        self.cursor.get() >= self.content.len()
    }
}

pub struct Lexer<'a> {
    buffer: LexBuffer<'a>,
    seeds: Vec<Box<dyn ReadableToken>>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        let buffer = LexBuffer {
            content,
            cursor: Cell::new(0),
        };
        buffer.trim();
        Lexer {
            buffer,
            seeds: vec![],
        }
    }

    pub fn push_seed(&mut self, seed: impl ReadableToken + 'static) -> &mut Self {
        self.seeds.push(Box::new(seed));
        self
    }

    pub fn exec(self) -> Result<TokenStream, Error> {
        let mut tokens = vec![];

        while !self.buffer.end() {
            let mut matched = false;
            for seed in self.seeds.iter() {
                if let Some(result) = seed.parse(&self.buffer) {
                    let item = result.map_err(|e| self.buffer.error_head(e))?;
                    tokens.push(item);
                    matched = true;
                    break;
                }
            }

            if !matched {
                return Err(self.buffer.error_head(ParseError::UnknownToken));
            }
        }

        Ok(TokenStream::new(tokens))
    }
}

#[test]
fn test_lex() {
    let mut lexer = Lexer::new("  +  *  +  ");
    lexer.push_seed(SymbolToken::Add);

    let result = lexer.exec().unwrap();

    assert_eq!(result.len(), 3);

    assert_eq!(result.to_string(), "+*+".to_string());
}
