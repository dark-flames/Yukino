use yukino::cli_entry;

cli_entry!(
    resolver = [],
    after_setup = ["cargo fmt"],
    entity_files = ["schema/schema.rs"],
    output_file = "src/schema.rs"
);
