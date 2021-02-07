#![feature(associated_type_defaults)]
#![feature(unsafe_cell_raw_get)]

pub mod annotations;
mod association;
pub mod definitions;
mod interface;
pub mod repository;
pub mod resolver;
mod transaction;
pub mod types;
pub mod query;

pub use interface::*;
pub use transaction::*;

pub mod collection {
    pub use super::association::AssociatedEntity;
}
