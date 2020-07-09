extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(
    Yukino,
    attributes(Table, Index, Column, Id, Ignore, Association, InverseAssociation)
)]
pub fn yukino_entity_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
