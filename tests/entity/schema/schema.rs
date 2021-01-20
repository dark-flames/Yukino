use std::collections::HashMap;

#[Entity(name = "foo")]
struct Foo {
    integer: u32,
    int16: i16,
    list: Vec<String>,
    map: HashMap<String, String>,
    string: String,
}
