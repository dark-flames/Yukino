use yui::YuiEnumValue;

/// Value of index method in attribute `Index`.
/// Variants will be different in different platform:
/// * "hash" is not support on `sqlite`
/// * "gin", "sp_gin", "gist", "brin" is only available on `postgre-sql`
#[derive(YuiEnumValue, Clone, Eq, PartialEq, Debug)]
#[mod_path = "yukino::mapping"]
pub enum IndexMethod {
    BTree,
    #[cfg(any(feature = "mysql", feature = "postgre-sql"))]
    Hash,
    #[cfg(any(feature = "postgre-sql"))]
    Gin,
    #[cfg(any(feature = "postgre-sql"))]
    #[variant_value("sp_gin")]
    SPGin,
    #[cfg(any(feature = "postgre-sql"))]
    Gist,
    #[cfg(any(feature = "postgre-sql"))]
    Brin,
}

#[derive(YuiEnumValue, Clone, Eq, PartialEq, Debug)]
#[mod_path = "yukino::mapping"]
pub enum ReferenceAction {
    NoAction,
    Restrict,
    SetNull,
    SetDefault,
    Cascade,
}
