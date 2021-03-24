use iroha::ToTokens;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(ToTokens, Clone, Eq, PartialEq, Debug)]
#[Iroha(mod_path = "yukino::query::type_check")]
pub enum TypeKind {
    Numeric,
    String,
    Boolean,
    Null,
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
                TypeKind::Null => "Null",
                TypeKind::Others(s) => s
            }
        )
    }
}
