use yukino::cli_entry;
use yukino::mapping::{IntegerResolveCell, FloatResolveCell};


cli_entry!(
    resolver = [IntegerResolveCell, FloatResolveCell],
    entity_files = {
        "crate::entities" -> "src/entities.rs"
    },
    output_file = "src/schema.rs"
);
