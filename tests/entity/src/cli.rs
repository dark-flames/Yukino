use yukino::cli_entry;
use yukino::resolver::default_resolver::{
    CollectionFieldResolverSeed, NumericFieldResolverSeed, StringFieldResolverSeed,
};

cli_entry!(
    resolver = [
        NumericFieldResolverSeed,
        CollectionFieldResolverSeed,
        StringFieldResolverSeed
    ],
    after_setup = [
        "cargo fmt"
    ],
    entity_files = {
        "crate::entities" -> "src/entities.rs"
    },
    output_file = "src/schema.rs"
);
