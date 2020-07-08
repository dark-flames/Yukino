mod association;
mod error;
mod entity;
pub mod mapping;

pub use association::*;
pub use entity::*;
pub use mapping::definition::*;
pub use mapping::r#type::*;
pub use mapping::attribution::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
