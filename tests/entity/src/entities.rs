use std::collections::HashMap;
use yukino::Yukino;

#[derive(Yukino)]
#[Entity(name = "foo")]
pub struct Foo {
    pub(crate) integer: u32,
    pub(crate) int16: i16,
    pub(crate) list: Vec<String>,
    pub(crate) map: HashMap<String, String>,
    pub(crate) string: String,
}
