pub trait FetchStrategy {
    fn get_instance() -> Self;
}

#[allow(dead_code)]
pub struct Auto;

impl FetchStrategy for Auto {
    fn get_instance() -> Self {
        Auto
    }
}
