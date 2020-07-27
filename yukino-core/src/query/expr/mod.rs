mod expression;
mod function;
mod ident;
mod logical;
mod mathematical;
mod subquery;
mod value;

pub use expression::*;
pub use function::*;
pub use ident::*;
pub use logical::*;
pub use mathematical::*;
pub use subquery::*;
pub use value::*;
use syn::parse::{Parse, ParseBuffer};

pub(crate) trait Peekable: Parse {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool;
}
