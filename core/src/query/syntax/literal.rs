use proc_macro2::Ident;
use crate::query::syntax::expression::TypeFlag;


pub enum Literal {
    Immediate{content: String, ty: TypeFlag},
    External{ident: Ident, ty: TypeFlag}
}