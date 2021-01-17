impl crate::entities::Foo {
    pub fn get_integer_converter(
    ) -> yukino::resolver::default_resolver::UnsignedIntegerValueConverter {
        yukino::resolver::default_resolver::UnsignedIntegerValueConverter::new(
            "integer".to_string(),
            "crate::entities::Foo".to_string(),
            "integer".to_string(),
        )
    }
    pub fn get_int16_converter() -> yukino::resolver::default_resolver::SmallIntegerValueConverter {
        yukino::resolver::default_resolver::SmallIntegerValueConverter::new(
            "int16".to_string(),
            "crate::entities::Foo".to_string(),
            "int16".to_string(),
        )
    }
}
impl yukino::Entity for crate::entities::Foo {
    fn from_database_value(
        result: &std::collections::HashMap<String, yukino::types::DatabaseValue>,
    ) -> Result<Box<Self>, yukino::resolver::error::DataConvertError> {
        use yukino::resolver::ValueConverter;
        let integer = Self::get_integer_converter().to_value(result)?;
        let int16 = Self::get_int16_converter().to_value(result)?;
        Ok(Box::new(crate::entities::Foo { integer, int16 }))
    }
    fn to_database_value(
        &self,
    ) -> Result<
        std::collections::HashMap<String, yukino::types::DatabaseValue>,
        yukino::resolver::error::DataConvertError,
    > {
        let mut map = std::collections::HashMap::new();
        use yukino::resolver::ValueConverter;
        map.extend(Self::get_integer_converter().to_database_value_by_ref(&self.integer)?);
        map.extend(Self::get_int16_converter().to_database_value_by_ref(&self.int16)?);
        Ok(map)
    }
    fn get_definitions() -> Vec<yukino::definitions::TableDefinition> {
        vec![yukino::definitions::TableDefinition::new(
            "foo".to_string(),
            yukino::definitions::TableType::NormalEntityTable("crate::entities::Foo".to_string()),
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
                    "int16".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("int16".to_string()),
                    yukino::types::DatabaseType::SmallInteger,
                    false,
                    false,
                    false,
                ),
            ],
            vec![],
            vec![],
        )]
    }
}
