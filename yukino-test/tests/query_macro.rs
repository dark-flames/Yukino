#[macro_use]
extern crate yukino;
use yukino::query::AliasItem;

#[test]
fn test_alias() {
    assert_eq!(
        alias!(a.b.c as a1),
        AliasItem {
            path: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            alias: Some("a1".to_string())
        }
    );

    assert_eq!(
        alias!(b),
        AliasItem {
            path: vec!["b".to_string()],
            alias: None
        }
    )
}
