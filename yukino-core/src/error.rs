use std::error::Error;
use syn::export::fmt::Display;

pub trait RuntimeError: Error + Display {
    fn get_message(&self) -> String;
}