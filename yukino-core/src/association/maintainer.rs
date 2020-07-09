pub trait Maintainer {
    fn get_instance() -> Self;
}

#[allow(dead_code)]
pub struct Cascade;

impl Maintainer for Cascade {
    fn get_instance() -> Self {
        Cascade
    }
}
