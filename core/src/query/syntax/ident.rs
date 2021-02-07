use crate::query::syntax::expression::TypeFlag;

#[allow(dead_code)]
pub struct DatabaseIdent {
    segments: Vec<String>,
    ty: TypeFlag,
}
