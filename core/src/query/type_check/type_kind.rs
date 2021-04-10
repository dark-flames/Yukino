use crate::query::ast::Literal;
use iroha::ToTokens;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(ToTokens, Clone, Eq, PartialEq, Debug)]
#[Iroha(mod_path = "yukino::query::type_check")]
pub enum TypeKind {
    Numeric,
    String,
    Boolean,
    Object(String),
    Others(String),
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                TypeKind::Numeric => "Numeric",
                TypeKind::String => "String",
                TypeKind::Boolean => "Boolean",
                TypeKind::Object(name) => return write!(f, "Object({})", name),
                TypeKind::Others(s) => s,
            }
        )
    }
}

impl From<&Literal> for TypeKind {
    fn from(lit: &Literal) -> Self {
        match lit {
            Literal::Boolean(_) => TypeKind::Boolean,
            Literal::Float(_) | Literal::Integer(_) => TypeKind::Numeric,
            Literal::String(_) => TypeKind::String,
            Literal::External(e) => TypeKind::Others(format!("External Value: {}", e.ident)),
            Literal::Null(_) => TypeKind::Others("Null".to_string()),
        }
    }
}
