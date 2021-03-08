use crate::query::expr::binary::BinaryOperator;
use crate::query::expr::unary::UnaryOperator;
use crate::query::expr::{DatabaseIdent, FunctionCall, Literal};
use crate::query::parse::ParseBuffer;

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
    BitReverse,
    Ident,
    FunctionCall,
    Lit,
}

impl Precedence {
    pub fn peek(buffer: &ParseBuffer) -> Option<Self> {
        Some(if buffer.peek::<Literal>() {
            Precedence::Lit
        } else if buffer.peek::<FunctionCall>() {
            Precedence::FunctionCall
        } else if buffer.peek::<DatabaseIdent>() {
            Precedence::Ident
        } else if buffer.peek::<UnaryOperator>() {
            UnaryOperator::peek_operator(buffer).unwrap().precedence()
        } else if buffer.peek::<BinaryOperator>() {
            BinaryOperator::peek_operator(buffer).unwrap().precedence()
        } else {
            return None;
        })
    }
}
