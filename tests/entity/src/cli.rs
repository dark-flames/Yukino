use yukino::cli_entry;

cli_entry!(
    resolver = [],
    after_setup = ["cargo fmt"],
    entity_files = { "schema/entities.rs" },
    output_file = "src/schema.rs"
);
