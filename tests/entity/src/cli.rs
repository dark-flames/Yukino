use yukino::cli_entry;
use yukino::resolver::default_resolver::NumericFieldResolverSeed;

cli_entry!(
    resolver = [
        NumericFieldResolverSeed
    ],
    after_setup = [
        "cargo fmt"
    ],
    entity_files = {
        "crate::entities" -> "src/entities.rs"
    },
    output_file = "src/schema.rs"
);
