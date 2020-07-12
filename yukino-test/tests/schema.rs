use serde_json::json;
use std::collections::HashMap;
use yukino::mapping::DatabaseValue;
use yukino::Entity;
use yukino_test::entities::Foo;

#[test]
fn test_foo() {
    let key = [
        "integer".to_string(),
        "int16".to_string(),
        "vec".to_string(),
        "map".to_string(),
    ];
    let mut raw_data = HashMap::new();
    raw_data.insert(key[0].clone(), DatabaseValue::UnsignedInteger(114514));
    raw_data.insert(key[1].clone(), DatabaseValue::SmallInteger(1919));
    raw_data.insert(key[2].clone(), DatabaseValue::Json(json!(["a", "b"])));
    raw_data.insert(
        key[3].clone(),
        DatabaseValue::Json(json!({
            "a": 1,
            "b": 2
        })),
    );

    let object = *Foo::from_raw_result(&raw_data).unwrap();

    let raw_data = object.to_raw_value().unwrap();

    match raw_data.get(&key[0]).unwrap() {
        DatabaseValue::UnsignedInteger(value) => assert_eq!(*value, 114514),
        _ => panic!(),
    }

    match raw_data.get(&key[1]).unwrap() {
        DatabaseValue::SmallInteger(value) => assert_eq!(*value, 1919),
        _ => panic!(),
    }
    match raw_data.get(&key[2]).unwrap() {
        DatabaseValue::Json(value) => assert_eq!(*value, json!(["a", "b"])),
        _ => panic!(),
    }

    match raw_data.get(&key[3]).unwrap() {
        DatabaseValue::Json(value) => assert_eq!(
            *value,
            json!({
                "a": 1,
                "b": 2
            })
        ),
        _ => panic!(),
    }
}
