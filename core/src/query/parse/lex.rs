use crate::query::parse::error::{Error, ParseError};
use crate::query::parse::parser::TokenStream;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Chars;

#[derive(Clone)]
pub enum Token {
    Symbol(Symbol),
    Ident(Ident)
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Token::Symbol(symbol) => symbol.fmt(f),
            Token::Ident(ident) => ident.fmt(f)
        }
    }
}

pub trait ReadableToken: Display{
    fn parse(&self, buffer: &mut LexBuffer) -> Option<Result<Token, ParseError>>;

    fn seed() -> Self where Self: Sized;
}

macro_rules! symbol {
    (
        $(($token: tt $name: ident $pattern: expr)),*
    ) => {
        #[derive(Clone)]
        pub enum Symbol {
            $($name),*
        }

        impl ReadableToken for Symbol {
            fn parse(&self, buffer: &mut LexBuffer) -> Option<Result<Token, ParseError>> {
                let pattern = vec![
                    $((Symbol::$name, $pattern)),*
                ];
                for (token, regex) in pattern {
                    let result = Regex::new(regex).unwrap().captures(buffer.rest());
                    let chars = result.as_ref().map(
                        |caps| caps.get(0).unwrap().as_str().chars().count()
                    ).unwrap_or(0);
                    if result.is_some() {
                        buffer.eat(chars);
                        return Some(Ok(Token::Symbol(token)))
                    }
                }

                None
            }
            fn seed() -> Self where Self: Sized {
                Symbol::Add
            }
        }

        impl Display for Symbol {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "{}", match self {
                    $(Symbol::$name => $token),*
                })
            }
        }
    };
}

symbol! {
    ("+" Add r"^\+"),
    ("*" Mul r"^\*")
}

#[derive(Clone)]
pub struct Ident {
    inner: String
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.inner.fmt(f)
    }
}

impl ReadableToken for Ident {
    fn parse(&self, buffer: &mut LexBuffer<'a>) -> Option<Result<Token, ParseError>> {
        let pattern = Regex::new(r"^([a-zA-Z][a-zA-Z0-9_]*)|(_[a-zA-Z0-9_]*)").unwrap();

        let result = pattern.captures(buffer.rest())?;

        let inner = result.get(0).unwrap().as_str().to_string();

        buffer.eat(inner.len());

        Some(Ok(Token::Ident(Ident {
            inner
        })))
    }

    fn seed() -> Self where Self: Sized {
        Ident {
            inner: "".to_string()
        }
    }
}

pub struct LexBuffer<'a> {
    content: Chars<'a>,
}

impl<'a> LexBuffer<'a> {
    pub fn rest(&self) -> &str {
        self.content.as_str()
    }

    pub fn eat(&mut self, n: usize) {
        for _ in 0..n {
            self.content.next();
        }

        let whitespace_regex = Regex::new(r"^\s+").unwrap();
        let count = whitespace_regex
            .captures(self.rest())
            .map(|result| result.get(0).unwrap().as_str().chars().count())
            .unwrap_or(0);

        for _ in 0..count {
            self.content.next();
        }
    }

    pub fn error_head(&self, error: ParseError) -> Error {
        Error::head(self.rest(), error)
    }

    pub fn end(&self) -> bool {
        self.rest().is_empty()
    }
}

pub struct Lexer<'a> {
    buffer: LexBuffer<'a>,
    seeds: Vec<Box<dyn ReadableToken>>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        let mut buffer = LexBuffer {
            content: content.chars(),
        };
        buffer.eat(0);
        Lexer {
            buffer,
            seeds: vec![],
        }
    }

    pub fn push_seed(&mut self, seed: impl ReadableToken + 'static) -> &mut Self {
        self.seeds.push(Box::new(seed));
        self
    }

    pub fn exec(mut self) -> Result<TokenStream, Error> {
        let mut tokens = vec![];

        while !self.buffer.end() {
            let mut matched = false;
            for seed in self.seeds.iter() {
                if let Some(result) = seed.parse(&mut self.buffer) {
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
    let mut lexer = Lexer::new("__ident_a + ident_b * IdentC");
    lexer.push_seed(Symbol::seed());
    lexer.push_seed(Ident::seed());

    let result = lexer.exec().unwrap();

    assert_eq!(result.len(), 5);

    assert_eq!(result.to_string(), "__ident_a+ident_b*IdentC".to_string());
}
