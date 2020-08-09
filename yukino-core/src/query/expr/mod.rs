mod expression;
mod function;
mod ident;
mod mathematical;
mod subquery;
mod value;

pub use expression::*;
pub use function::*;
pub use ident::*;
pub use mathematical::*;
pub use subquery::*;
use syn::parse::{Parse, ParseBuffer};
pub use value::*;

pub(crate) trait Peekable: Parse {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool;
}
