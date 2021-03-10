#![allow(clippy::upper_case_acronyms)]

#[derive(Parser)]
#[grammar = "query/grammar.pest"]
pub struct Grammar;
