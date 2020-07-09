use std::collections::HashMap;
use yukino::Entity;
use yukino_test::entities::Foo;
use yukino::mapping::DatabaseValue;

#[test]
fn tes_foo() {
    let key_1 = "integer".to_string();
    let key_2 = "int16".to_string();
    let mut raw_data = HashMap::new();
    raw_data.insert(key_1.clone(), DatabaseValue::UnsignedInteger(114514));
    raw_data.insert(key_2.clone(), DatabaseValue::SmallInteger(1919));

    let object = *Foo::from_raw_result(&raw_data).unwrap();

    let raw_data = object.to_raw_value().unwrap();

    match raw_data.get(&key_1).unwrap() {
        DatabaseValue::UnsignedInteger(value) => assert_eq!(*value, 114514),
        _ => panic!()
    }

    match raw_data.get(&key_2).unwrap() {
        DatabaseValue::SmallInteger(value) => assert_eq!(*value, 1919),
        _ => panic!()
    }
}