#![feature(associated_type_defaults)]
#![feature(unsafe_cell_raw_get)]
#![feature(in_band_lifetimes)]

#[macro_use]
extern crate pest_derive;
extern crate pest;

pub mod annotations;
mod association;
pub mod definitions;
mod interface;
pub mod query;
pub mod repository;
pub mod resolver;
mod transaction;
pub mod types;

pub use interface::*;
pub use transaction::*;

pub mod collection {
    pub use super::association::AssociatedEntity;
}
