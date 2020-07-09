use syn::{parse_quote, DeriveInput};
use yukino_core::mapping::definition::{ColumnDefinition, IndexDefinition, TableDefinition};
use yukino_core::mapping::resolver::{
    CellResolver, ConstructableCell, FieldResolveCell, NumericResolveCell,
};
use yukino_core::mapping::DatabaseType;
use yukino_core::mapping::IndexMethod;

#[test]
fn test_numeric() {
    let seeds: Vec<Box<dyn FieldResolveCell>> = vec![Box::new(NumericResolveCell::get_seed())];

    let mut resolver = CellResolver::new(seeds);

    let input: DeriveInput = parse_quote! {
        #[Table(name="test_table", indexes(
            integer(columns("integer"), unique=true)
        ))]
        pub struct Test {
            integer: u32
        }
    };

    if let Err(e) = resolver.parse(input, "test") {
        panic!(e.to_string())
    };

    let definitions = resolver.get_definitions().unwrap();

    let expect_definition = TableDefinition {
        name: "test_table".to_string(),
        indexes: vec![IndexDefinition {
            name: "integer".to_string(),
            method: IndexMethod::BTree,
            columns: vec!["integer".to_string()],
            unique: true,
        }],
        columns: vec![
            ColumnDefinition {
                name: "__test_id".to_string(),
                column_type: DatabaseType::String,
                unique: true,
                auto_increase: false,
                is_primary_key: true,
            },
            ColumnDefinition {
                name: "integer".to_string(),
                column_type: DatabaseType::UnsignedInteger,
                unique: false,
                auto_increase: false,
                is_primary_key: false,
            },
        ],
        foreign_keys: vec![],
    };

    assert_eq!(definitions[0], expect_definition);
    print!("{}", resolver.get_implements().unwrap().to_string())
}
