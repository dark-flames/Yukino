use yukino::cli_entry;
use yukino::mapping::resolver::{FloatResolveCell, IntegerResolveCell};

cli_entry!(
    resolver = [IntegerResolveCell, FloatResolveCell],
    entity_files = {
        "crate::entities" -> "src/entities.rs"
    },
    output_file = "src/schema.rs",
    after_setup = "cargo fmt"
);
