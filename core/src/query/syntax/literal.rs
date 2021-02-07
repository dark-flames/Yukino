use crate::query::syntax::expression::TypeFlag;
use proc_macro2::Ident;

pub enum Literal {
    Immediate { content: String, ty: TypeFlag },
    External { ident: Ident, ty: TypeFlag },
}
