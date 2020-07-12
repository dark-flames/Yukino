use yukino::cli_entry;
use yukino::mapping::resolver::{CollectionResolveCell, NumericResolveCell};

cli_entry!(
    resolver = [
        NumericResolveCell,
        CollectionResolveCell
    ],
    after_setup = [
        "cargo fmt"
    ],
    entity_files = {
        "crate::entities" -> "src/entities.rs"
    },
    output_file = "src/schema.rs"
);
