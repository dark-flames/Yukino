use std::collections::HashMap;
use yukino::collection::AssociatedEntity;

#[Entity(name = "foo")]
pub struct Foo {
    integer: u32,
    int16: i16,
    list: Vec<String>,
    map: HashMap<String, String>,
    string: String,
    #[Association(mapped_by("id"))]
    bar: AssociatedEntity<Bar>
}

#[Entity(name = "bar")]
pub struct Bar {
    #[ID]
    id: u64
}
