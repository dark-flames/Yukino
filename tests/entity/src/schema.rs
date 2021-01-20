#![allow(unknown_lints)]
struct FooInner {
    integer: u32,
    int16: i16,
    list: Vec<String>,
    map: std::collections::HashMap<String, String>,
    string: String,
}
impl FooInner {
    pub fn get_integer_converter(
    ) -> yukino::resolver::field_resolver_seeds::UnsignedIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::UnsignedIntegerValueConverter::new(
            false,
            "integer".to_string(),
            "Foo".to_string(),
            "integer".to_string(),
        )
    }
    pub fn get_map_converter() -> yukino::resolver::field_resolver_seeds::MapValueConverter {
        yukino::resolver::field_resolver_seeds::MapValueConverter::new(
            "Foo".to_string(),
            "map".to_string(),
            "map".to_string(),
        )
    }
    pub fn get_int16_converter(
    ) -> yukino::resolver::field_resolver_seeds::SmallIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::SmallIntegerValueConverter::new(
            false,
            "int16".to_string(),
            "Foo".to_string(),
            "int16".to_string(),
        )
    }
    pub fn get_string_converter() -> yukino::resolver::field_resolver_seeds::StringValueConverter {
        yukino::resolver::field_resolver_seeds::StringValueConverter::new(
            false,
            "Foo".to_string(),
            "string".to_string(),
            "string".to_string(),
        )
    }
    pub fn get_list_converter() -> yukino::resolver::field_resolver_seeds::ListValueConverter {
        yukino::resolver::field_resolver_seeds::ListValueConverter::new(
            "Foo".to_string(),
            "list".to_string(),
            "list".to_string(),
        )
    }
}
impl yukino::Entity for FooInner {
    fn from_database_value(
        result: &std::collections::HashMap<String, yukino::types::DatabaseValue>,
    ) -> Result<Box<Self>, yukino::resolver::error::DataConvertError> {
        use yukino::resolver::ValueConverter;
        let integer = Self::get_integer_converter().to_field_value(result)?;
        let map = Self::get_map_converter().to_field_value(result)?;
        let int16 = Self::get_int16_converter().to_field_value(result)?;
        let string = Self::get_string_converter().to_field_value(result)?;
        let list = Self::get_list_converter().to_field_value(result)?;
        Ok(Box::new(FooInner {
            integer,
            map,
            int16,
            string,
            list,
        }))
    }
    fn to_database_values(
        &self,
    ) -> Result<
        std::collections::HashMap<String, yukino::types::DatabaseValue>,
        yukino::resolver::error::DataConvertError,
    > {
        let mut map = std::collections::HashMap::new();
        use yukino::resolver::ValueConverter;
        map.extend(Self::get_integer_converter().to_database_values_by_ref(&self.integer)?);
        map.extend(Self::get_map_converter().to_database_values_by_ref(&self.map)?);
        map.extend(Self::get_int16_converter().to_database_values_by_ref(&self.int16)?);
        map.extend(Self::get_string_converter().to_database_values_by_ref(&self.string)?);
        map.extend(Self::get_list_converter().to_database_values_by_ref(&self.list)?);
        Ok(map)
    }
    fn get_definitions() -> Vec<yukino::definitions::TableDefinition> {
        vec![yukino::definitions::TableDefinition::new(
            "foo".to_string(),
            yukino::definitions::TableType::NormalEntityTable("Foo".to_string()),
            vec![
                yukino::definitions::ColumnDefinition::new(
                    "__foo_id".to_string(),
                    yukino::definitions::ColumnType::VisualColumn,
                    yukino::types::DatabaseType::String,
                    true,
                    false,
                    true,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "integer".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("integer".to_string()),
                    yukino::types::DatabaseType::UnsignedInteger,
                    false,
                    false,
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "map".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("map".to_string()),
                    yukino::types::DatabaseType::Json,
                    false,
                    false,
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "int16".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("int16".to_string()),
                    yukino::types::DatabaseType::SmallInteger,
                    false,
                    false,
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "string".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("Foo".to_string()),
                    yukino::types::DatabaseType::String,
                    false,
                    false,
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "list".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("list".to_string()),
                    yukino::types::DatabaseType::Json,
                    false,
                    false,
                    false,
                ),
            ],
            vec![],
            vec![],
        )]
    }
    fn primary_key_values(
        &self,
    ) -> Result<
        std::collections::HashMap<String, yukino::types::DatabaseValue>,
        yukino::resolver::error::DataConvertError,
    > {
        let map = std::collections::HashMap::new();
        Ok(map)
    }
}
yukino::impl_entity_proxy!(Foo);
