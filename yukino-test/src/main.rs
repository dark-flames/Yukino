use yukino::cli_entry;
use yukino::mapping::resolver::NumericResolveCell;

cli_entry!(
    resolver = [
        NumericResolveCell
    ],
    after_setup = [
        "cargo fmt"
    ],
    entity_files = {
        "crate::entities" -> "src/entities.rs"
    },
    output_file = "src/schema.rs"
);
