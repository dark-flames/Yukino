pub trait FetchStrategy {}

#[allow(dead_code)]
pub struct Auto;

impl FetchStrategy for Auto {}