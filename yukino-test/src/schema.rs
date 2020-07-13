impl crate::entities::Foo {
    pub fn get_int16_converter() -> yukino::mapping::resolver::SmallIntegerValueConverter {
        yukino::mapping::resolver::SmallIntegerValueConverter::new("int16".to_string())
    }
    pub fn get_array_converter() -> yukino::mapping::resolver::ArrayValueConverter {
        yukino::mapping::resolver::ArrayValueConverter::new("array".to_string())
    }
    pub fn get_integer_converter() -> yukino::mapping::resolver::UnsignedIntegerValueConverter {
        yukino::mapping::resolver::UnsignedIntegerValueConverter::new("integer".to_string())
    }
    pub fn get_map_converter() -> yukino::mapping::resolver::MapValueConverter {
        yukino::mapping::resolver::MapValueConverter::new("map".to_string())
    }
}
impl yukino::Entity for crate::entities::Foo {
    fn from_database_value(
        result: &std::collections::HashMap<String, yukino::mapping::DatabaseValue>,
    ) -> Result<Box<Self>, yukino::ParseError> {
        use yukino::mapping::resolver::ValueConverter;
        let int16 = Self::get_int16_converter().to_value(result)?;
        let array = Self::get_array_converter().to_value(result)?;
        let integer = Self::get_integer_converter().to_value(result)?;
        let map = Self::get_map_converter().to_value(result)?;
        Ok(Box::new(crate::entities::Foo {
            int16,
            array,
            integer,
            map,
        }))
    }
    fn to_database_value(
        &self,
    ) -> Result<std::collections::HashMap<String, yukino::mapping::DatabaseValue>, yukino::ParseError>
    {
        let mut map = std::collections::HashMap::new();
        use yukino::mapping::resolver::ValueConverter;
        map.extend(Self::get_int16_converter().to_database_value(&self.int16)?);
        map.extend(Self::get_array_converter().to_database_value(&self.array)?);
        map.extend(Self::get_integer_converter().to_database_value(&self.integer)?);
        map.extend(Self::get_map_converter().to_database_value(&self.map)?);
        Ok(map)
    }
    fn get_definitions(&self) -> Vec<yukino::mapping::definition::TableDefinition> {
        vec![yukino::mapping::definition::TableDefinition::new(
            "foo".to_string(),
            vec![yukino::mapping::definition::IndexDefinition::new(
                "integer".to_string(),
                yukino::mapping::IndexMethod::BTree,
                vec!["integer".to_string()],
                true,
            )],
            vec![
                yukino::mapping::definition::ColumnDefinition::new(
                    "__foo_id".to_string(),
                    yukino::mapping::DatabaseType::String,
                    true,
                    false,
                    true,
                ),
                yukino::mapping::definition::ColumnDefinition::new(
                    "int16".to_string(),
                    yukino::mapping::DatabaseType::SmallInteger,
                    false,
                    false,
                    false,
                ),
                yukino::mapping::definition::ColumnDefinition::new(
                    "array".to_string(),
                    yukino::mapping::DatabaseType::Json,
                    false,
                    false,
                    false,
                ),
                yukino::mapping::definition::ColumnDefinition::new(
                    "integer".to_string(),
                    yukino::mapping::DatabaseType::UnsignedInteger,
                    false,
                    false,
                    false,
                ),
                yukino::mapping::definition::ColumnDefinition::new(
                    "map".to_string(),
                    yukino::mapping::DatabaseType::Json,
                    false,
                    false,
                    false,
                ),
            ],
            vec![],
        )]
    }
}
