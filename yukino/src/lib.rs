extern crate yukino_core;

pub use yukino_cli::{cli_entry, CommandLineEntry};
pub use yukino_core::*;
pub use yukino_derive::Yukino;

mod query_macro {
    pub use yukino_derive::assignment;
}
