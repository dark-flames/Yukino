mod query;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use crate::query::FieldAssignments;

#[proc_macro_derive(
    Yukino,
    attributes(Table, Index, Column, Id, Ignore, Association, InverseAssociation)
)]
pub fn entity_derive(_: TokenStream) -> TokenStream {
    TokenStream::new()
}


#[proc_macro]
pub fn assignment(input: TokenStream) -> TokenStream {
    let assignments = parse_macro_input!(input as FieldAssignments);

    TokenStream::from(assignments.to_assignment_items())
}