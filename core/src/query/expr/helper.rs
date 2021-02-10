use syn::parse::{Parse, ParseBuffer};

pub(crate) trait Peekable: Parse {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool;
}