mod association;
mod error;
mod entity;
pub mod mapping;

pub use association::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
