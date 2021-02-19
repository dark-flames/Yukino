use crate::query::parse::error::{Error, ParseError};
use crate::query::parse::parser::TokenStream;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Chars;

#[derive(Clone)]
pub enum Token {
    Symbol(Symbol),
    Ident(Ident),
    Keyword(Keyword),
    Lit(Literal),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Token::Symbol(symbol) => symbol.fmt(f),
            Token::Ident(ident) => ident.fmt(f),
            Token::Keyword(keyword) => keyword.fmt(f),
            Token::Lit(lit) => lit.fmt(f),
        }
    }
}

impl ReadableToken for Token {
    fn parse(&self, buffer: &mut LexBuffer<'a>) -> Option<Result<Token, ParseError>> {
        let seeds = [
            Keyword::seed(),
            Symbol::seed(),
            Ident::seed(),
            Literal::seed(),
        ];

        for seed in seeds.iter() {
            if let Some(result) = seed.parse(buffer) {
                return Some(result);
            }
        }

        Some(Err(ParseError::UnknownToken))
    }

    fn seed() -> Box<dyn ReadableToken>
    where
        Self: Sized,
    {
        Box::new(Token::Symbol(Symbol::Add))
    }
}

pub trait ReadableToken: Display {
    fn parse(&self, buffer: &mut LexBuffer) -> Option<Result<Token, ParseError>>;

    fn seed() -> Box<dyn ReadableToken>
    where
        Self: Sized;
}

macro_rules! symbols {
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
            fn seed() -> Box<dyn ReadableToken>
                where
                    Self: Sized {
                Box::new(Symbol::Add)
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

symbols! {
    ("+" Add r"^\+"),
    ("*" Mul r"^\*"),
    ("." Dot r"^\.")
}

macro_rules! keywords {
    (
        $(($token: tt $name: ident)),*
    ) => {
        #[derive(Clone)]
        pub enum Keyword {
            $($name),*
        }

        impl ReadableToken for Keyword {
            fn parse(&self, buffer: &mut LexBuffer) -> Option<Result<Token, ParseError>> {
                let patterns = vec![
                    $((Keyword::$name, $token.to_string())),*
                ];
                for (token, pattern) in patterns {
                    let head: String = buffer.rest().chars().take(pattern.len()).collect();

                    if head.to_lowercase() == pattern {
                        buffer.eat(head.len());
                        return Some(Ok(Token::Keyword(token)));
                    };
                };

                None
            }
            fn seed() -> Box<dyn ReadableToken>
                where
                    Self: Sized {
                Box::new(Keyword::Select)
            }
        }

        impl Display for Keyword {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "{}", match self {
                    $(Keyword::$name => $token),*
                })
            }
        }
    };
}

keywords! {
    ("select" Select)
}

#[derive(Clone)]
pub struct Ident {
    inner: String,
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

        Some(Ok(Token::Ident(Ident { inner })))
    }

    fn seed() -> Box<dyn ReadableToken>
    where
        Self: Sized,
    {
        Box::new(Ident {
            inner: "".to_string(),
        })
    }
}

#[derive(Clone)]
pub enum Literal {
    Int(usize),
    Float(String),
    Bool(bool),
    String(String),
    External(Ident),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Literal::Int(v) => v.fmt(f),
            Literal::Float(s) => s.fmt(f),
            Literal::Bool(v) => v.fmt(f),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::External(ident) => write!(f, "${}", ident),
        }
    }
}

impl ReadableToken for Literal {
    fn parse(&self, buffer: &mut LexBuffer<'a>) -> Option<Result<Token, ParseError>> {
        if let Some(result) = Regex::new(r"^\d+.\d+").unwrap().captures(buffer.rest()) {
            let inner = result.get(0).unwrap().as_str().to_string();

            buffer.eat(inner.len());

            Some(Ok(Token::Lit(Literal::Float(inner))))
        } else if let Some(result) = Regex::new(r"^(true)|(false)")
            .unwrap()
            .captures(buffer.rest())
        {
            let inner = result.get(0).unwrap().as_str().to_string();

            buffer.eat(inner.len());

            Some(Ok(if inner.as_str() == "true" {
                Token::Lit(Literal::Bool(true))
            } else {
                Token::Lit(Literal::Bool(false))
            }))
        } else if let Some(result) = Regex::new("^\"(\\w+)\"").unwrap().captures(buffer.rest()) {
            let size = result.get(0).unwrap().as_str().chars().count();
            let inner = result.get(1).unwrap().as_str().to_string();

            buffer.eat(size);

            Some(Ok(Token::Lit(Literal::String(inner))))
        } else if let Some(result) = Regex::new(r"^\d+").unwrap().captures(buffer.rest()) {
            let inner = result.get(0).unwrap().as_str().to_string();

            buffer.eat(inner.len());

            Some(Ok(Token::Lit(Literal::Int(inner.parse().unwrap()))))
        } else {
            None
        }
    }

    fn seed() -> Box<dyn ReadableToken>
    where
        Self: Sized,
    {
        Box::new(Literal::Bool(false))
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
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        let mut buffer = LexBuffer {
            content: content.chars(),
        };
        buffer.eat(0);
        Lexer { buffer }
    }

    pub fn exec(mut self) -> Result<TokenStream, Error> {
        let mut tokens = vec![];

        while !self.buffer.end() {
            tokens.push(
                Token::seed()
                    .parse(&mut self.buffer)
                    .unwrap()
                    .map_err(|e| self.buffer.error_head(e))?,
            )
        }

        Ok(TokenStream::new(tokens))
    }
}

#[test]
fn test_lex() {
    use std::str::FromStr;

    let result = TokenStream::from_str(
        "sElect __ident_a + ident_b * IdentC + 1 + \"sdasds\""
    ).unwrap();

    assert_eq!(result.len(), 10);

    assert_eq!(
        result.to_string(),
        "select __ident_a + ident_b * IdentC + 1 + \"sdasds\" ".to_string()
    );
}
