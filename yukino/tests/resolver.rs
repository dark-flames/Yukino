use yukino::mapping::resolver::IntegerResolveCell;
use yukino::mapping::{ConstructableCell, CellResolver};
use yukino::mapping::FieldResolveCell;
use syn::{parse_quote, DeriveInput};


#[test]
fn test_integer() {
    let seeds:Vec<Box<dyn FieldResolveCell>> = vec![
        Box::new(IntegerResolveCell::get_seed())
    ];

    let mut resolver = CellResolver::new(seeds);

    let input: DeriveInput = parse_quote!{
        #[Table(name="test_table")]
        pub struct Test {
            #[Column(unique=true)]
            integer: u32
        }
    };

    if let Err(e) = resolver.parse(input, "test") {
        panic!(e.to_string())
    };

    let definitions = resolver.get_definitions().unwrap();

    let expect_definition = TableDefinition {
        name: "test_table".to_string(),
        indexes: vec![
            IndexDefinition {
                name: "integer".to_string(),
                method: IndexMethod::BTree,
                columns: vec!["integer".to_string()],
                unique: true
            }
        ],
        columns: vec![
            ColumnDefinition {
                name: "__test_id".to_string(),
                column_type: DatabaseType::String,
                unique: true,
                auto_increase: false,
                is_primary_key: true
            },
            ColumnDefinition {
                name: "integer".to_string(),
                column_type: DatabaseType::UnsignedInteger,
                unique: false,
                auto_increase: false,
                is_primary_key: false
            }
        ],
        foreign_keys: vec![]
    };

    assert_eq!(definitions[0], expect_definition);
}