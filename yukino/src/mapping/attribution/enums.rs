use yui::YuiEnumValue;

#[derive(YuiEnumValue, Clone, PartialEq)]
pub enum IndexMethod {
    BTree,
    #[cfg(any(feature = "mysql", feature = "postgre-sql"))]
    Hash,
    #[cfg(any(feature = "postgre-sql"))]
    Gin,
    #[cfg(any(feature = "postgre-sql"))]
    #[variant_value("sp-gin")]
    SPGin,
    #[cfg(any(feature = "postgre-sql"))]
    Gist,
    #[cfg(any(feature = "postgre-sql"))]
    Brin,
}