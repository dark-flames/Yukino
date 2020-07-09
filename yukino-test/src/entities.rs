use yukino::Yukino;

#[derive(Yukino)]
#[Table(name = "foo", indexes(integer(columns("integer"), unique = true)))]
#[allow(dead_code)]
pub struct Foo {
    pub(super) integer: u32,
    pub(super) int16: i16,
}

#[derive(Yukino)]
#[Table(name = "bar", indexes(float(columns("float"), unique = true)))]
#[allow(dead_code)]
pub struct Bar {
    pub(super) float: f32,
    pub(super) float64: f64,
}
