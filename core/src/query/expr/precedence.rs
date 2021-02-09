use proc_macro2::Ident;
use syn::parse::ParseBuffer;
use syn::{Ident as IdentMark, Token};

#[allow(dead_code)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Precedence {
    None,
    Or,
    Xor,
    And,
    Not,
    Comparison,
    BitOr,
    BitAnd,
    BitShift,
    Add,
    Term,
    BitXor,
    BitInverse,
}

#[allow(dead_code)]
impl Precedence {
    pub fn peek<'a>(input: &'a ParseBuffer<'a>) -> Option<Self> {
        if input.peek(Token![^]) {
            Some(Precedence::BitXor)
        } else if input.peek(Token![*]) | input.peek(Token![/]) | input.peek(Token![%]) {
            Some(Precedence::Term)
        } else if input.peek(Token![+]) | input.peek(Token![-]) {
            Some(Precedence::Add)
        } else if input.peek(Token![>>]) | input.peek(Token![<<]) {
            Some(Precedence::BitShift)
        } else if input.peek(Token![&]) {
            Some(Precedence::BitAnd)
        } else if input.peek(Token![|]) {
            Some(Precedence::BitOr)
        } else if input.peek(Token![<=])
            | input.peek(Token![>=])
            | input.peek(Token![<])
            | input.peek(Token![>])
            | input.peek(Token![==])
            | input.peek(Token![!=])
        {
            Some(Precedence::Comparison)
        } else if input.peek(IdentMark) {
            let ident: Ident = input.fork().parse().ok()?;
            let ident_str = ident.to_string().to_lowercase();
            match ident_str.as_str() {
                "or" => Some(Precedence::Or),
                "xor" => Some(Precedence::Xor),
                "and" => Some(Precedence::And),
                "not" => Some(Precedence::Not),
                _ => None,
            }
        } else if input.peek(Token![||]) {
            Some(Precedence::Or)
        } else if input.peek(Token![&&]) {
            Some(Precedence::And)
        } else {
            None
        }
    }
}
