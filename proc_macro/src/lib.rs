extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(
    Yukino,
    attributes(Entity, Index, Field, ID, Ignore, Association, InverseAssociation)
)]
pub fn entity_derive(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
