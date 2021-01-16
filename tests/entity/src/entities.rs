use yukino::Yukino;

#[derive(Yukino)]
#[Entity(name = "foo")]
#[allow(dead_code)]
pub struct Foo {
    pub(crate) integer: u32,
    pub(crate) int16: i16,
}
