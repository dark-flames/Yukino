use yukino::cli_entry;
use yukino::mapping::resolver::NumericResolveCell;

cli_entry!(
    resolver = [NumericResolveCell],
    entity_files = {
        "crate::entities" -> "src/entities.rs"
    },
    output_file = "src/schema.rs",
    after_setup = "cargo fmt"
);
