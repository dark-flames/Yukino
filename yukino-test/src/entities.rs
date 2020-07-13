use std::collections::HashMap;
use yukino::Yukino;

#[derive(Yukino)]
#[Table(name = "foo", indexes(integer(columns("integer"), unique = true)))]
#[allow(dead_code)]
pub struct Foo {
    pub(super) integer: u32,
    pub(super) int16: i16,
    pub(super) array: Vec<String>,
    pub(super) map: HashMap<String, i32>,
}
