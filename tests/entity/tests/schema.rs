use std::collections::HashMap;
use yukino_test_entity::entities::Foo;
use yukino::types::DatabaseValue;
use yukino::Entity;

#[test]
pub fn test_foo() {
    let key = [
        "integer".to_string(),
        "int16".to_string()
    ];

    let mut raw_data = HashMap::new();
    raw_data.insert(key[0].clone(), DatabaseValue::UnsignedInteger(114514));
    raw_data.insert(key[1].clone(), DatabaseValue::SmallInteger(1919));

    let object = *Foo::from_database_value(&raw_data).unwrap();

    let raw_data = object.to_database_value().unwrap();

    match raw_data.get(&key[0]).unwrap() {
        DatabaseValue::UnsignedInteger(value) => assert_eq!(value.clone(), 114514),
        _ => panic!(),
    }

    match raw_data.get(&key[1]).unwrap() {
        DatabaseValue::SmallInteger(value) => assert_eq!(value.clone(), 1919),
        _ => panic!(),
    }
}