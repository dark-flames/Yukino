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

    for definition in definitions {
        println!("{:?}", definition)
    }
}