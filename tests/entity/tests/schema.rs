use serde_json::json;
use std::collections::HashMap;
use yukino::types::DatabaseValue;
use yukino::Entity;
use yukino_test_entity::entities::Foo;

#[test]
pub fn test_foo() {
    let data = [
        ("integer", DatabaseValue::UnsignedInteger(114514)),
        ("int16", DatabaseValue::SmallInteger(1919)),
        ("list", DatabaseValue::Json(json!(["114515", "1919"]))),
        (
            "map",
            DatabaseValue::Json(json!({
                "114": "514",
                "1919": "810"
            })),
        ),
        ("string", DatabaseValue::String("田所浩二".to_string())),
    ];

    let mut raw_data = HashMap::new();

    for (key, value) in data.iter() {
        raw_data.insert(key.to_string(), value.clone());
    }

    let object = *Foo::from_database_value(&raw_data).unwrap();

    let result = object.to_database_values().unwrap();

    match result.get(data[0].0).unwrap() {
        DatabaseValue::UnsignedInteger(value) => assert_eq!(value.clone(), 114514),
        _ => panic!(),
    }

    match result.get(data[1].0).unwrap() {
        DatabaseValue::SmallInteger(value) => assert_eq!(value.clone(), 1919),
        _ => panic!(),
    }
    match result.get(data[2].0).unwrap() {
        DatabaseValue::Json(value) => assert_eq!(value.clone(), json!(["114515", "1919"])),
        _ => panic!(),
    }

    match result.get(data[3].0).unwrap() {
        DatabaseValue::Json(value) => assert_eq!(
            value.clone(),
            json!({
                "114": "514",
                "1919": "810"
            })
        ),
        _ => panic!(),
    }

    match result.get(data[4].0).unwrap() {
        DatabaseValue::String(value) => assert_eq!(value.clone(), "田所浩二".to_string()),
        _ => panic!(),
    }
}
