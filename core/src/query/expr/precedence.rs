use crate::query::parse::{ParseBuffer};
use crate::query::expr::{FunctionCall, DatabaseIdent, Literal};
use crate::query::expr::mathematical::UnaryOperator;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Precedence {
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
    Ident,
    FunctionCall,
    Lit
}

impl Precedence {
    pub fn peek(buffer: &ParseBuffer) -> Self {
        if buffer.peek::<Literal>() {
            Precedence::Lit
        } else if buffer.peek::<FunctionCall>() {
            Precedence::FunctionCall
        } else if buffer.peek::<DatabaseIdent>() {
            Precedence::Ident
        } else if buffer.peek::<UnaryOperator>() {
            UnaryOperator::peek_operator(buffer).unwrap().precedence()
        } else {
            unreachable!()
        }
    }
}