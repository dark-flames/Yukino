mod entry;
mod error;
mod resolver;

pub use entry::*;
pub use error::*;
pub use resolver::*;

#[macro_export]
macro_rules! cli_entry {
    (
        resolver = [$($resolver: ident),*],
        after_setup = [$($after_setup: literal),*],
        entity_files = {
            $($mod_path: literal -> $file: literal),*
        },
        output_file = $output_path: literal
    ) => {
        use yukino::resolver::ConstructableFieldResolverSeed;
        use yukino::CommandLineEntry;
        use std::collections::HashMap;
        pub fn main() {
            let crate_path = env!("CARGO_MANIFEST_DIR");
            CommandLineEntry::new(
                vec![$(Box::new($resolver::new())),*],
                {
                    let mut map = HashMap::new();
                    $(
                        map.insert($mod_path, format!("{}/{}", crate_path, $file));
                    )*
                    map
                },
                format!("{}/{}", crate_path, $output_path),
                vec! [
                    $($after_setup),*
                ]
            ).process();
        }
    };
    (
        resolver = [$($resolver: ident),*],
        entity_files = {
            $($mod_path: literal -> $file: literal),*
        },
        output_file = $output_path: literal
    ) => {
        cli_entry!(
            resolver = [$($resolver: ident),*],
            after_setup = [],
            entity_files = {
                $($mod_path: literal -> $file: literal),*
            },
            output_file = $output_path: literal
        );
    }
}
