use float_eq::float_eq;

#[derive(Debug)]
pub enum Literal {
    Bool(bool),
    Int(usize),
    Float(f64),
    Str(String),
    Char(char),
    External(String),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Literal::Bool(x) => matches!(other, Literal::Bool(y) if x== y),
            Literal::Int(x) => matches!(other, Literal::Int(y) if x== y),
            Literal::Str(x) => matches!(other, Literal::Str(y) if x== y),
            Literal::Char(x) => matches!(other, Literal::Char(y) if x== y),
            Literal::External(x) => matches!(other, Literal::External(y) if x== y),
            Literal::Float(x) => matches!(other, Literal::Float(y) if float_eq!(x, y, ulps <= 4)),
        }
    }
}

impl Eq for Literal {}