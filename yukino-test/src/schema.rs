impl yukino::Entity for crate::entities::Foo {
    fn from_raw_result(
        result: &std::collections::HashMap<String, yukino::mapping::DatabaseValue>,
    ) -> Result<Box<Self>, yukino::ParseError> {
        let integer = match {
            let column_name = "integer".to_string();
            result.get(&column_name)
        } {
            Some(yukino::mapping::DatabaseValue::UnsignedInteger(integer)) => Ok(*integer),
            _ => Err(yukino::ParseError::new(
                "Unexpected DatabaseValue on field crate::entities::Foo",
            )),
        }?;
        let int16 = match {
            let column_name = "int16".to_string();
            result.get(&column_name)
        } {
            Some(yukino::mapping::DatabaseValue::SmallInteger(integer)) => Ok(*integer),
            _ => Err(yukino::ParseError::new(
                "Unexpected DatabaseValue on field crate::entities::Foo",
            )),
        }?;
        Ok(Box::new(crate::entities::Foo { integer, int16 }))
    }
    fn to_raw_value(
        &self,
    ) -> Result<std::collections::HashMap<String, yukino::mapping::DatabaseValue>, yukino::ParseError>
    {
        let mut database_value = std::collections::HashMap::new();
        database_value.insert(
            "integer".to_string(),
            yukino::mapping::DatabaseValue::UnsignedInteger(self.integer),
        );
        database_value.insert(
            "int16".to_string(),
            yukino::mapping::DatabaseValue::SmallInteger(self.int16),
        );
        Ok(database_value)
    }
    fn get_definitions(&self) -> Vec<yukino::mapping::definition::TableDefinition> {
        vec![yukino::mapping::definition::TableDefinition::new(
            "foo".to_string(),
            vec![yukino::mapping::definition::IndexDefinition::new(
                "integer".to_string(),
                { yukino::mapping::IndexMethod::BTree },
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
                    "integer".to_string(),
                    yukino::mapping::DatabaseType::UnsignedInteger,
                    false,
                    false,
                    false,
                ),
                yukino::mapping::definition::ColumnDefinition::new(
                    "int16".to_string(),
                    yukino::mapping::DatabaseType::Integer,
                    false,
                    false,
                    false,
                ),
            ],
            vec![],
        )]
    }
}
impl yukino::Entity for crate::entities::Bar {
    fn from_raw_result(
        result: &std::collections::HashMap<String, yukino::mapping::DatabaseValue>,
    ) -> Result<Box<Self>, yukino::ParseError> {
        let float = match {
            let column_name = "float".to_string();
            result.get(&column_name)
        } {
            Some(yukino::mapping::DatabaseValue::Float(integer)) => Ok(*integer),
            _ => Err(yukino::ParseError::new(
                "Unexpected DatabaseValue on field crate::entities::Bar",
            )),
        }?;
        let float64 = match {
            let column_name = "float64".to_string();
            result.get(&column_name)
        } {
            Some(yukino::mapping::DatabaseValue::Double(integer)) => Ok(*integer),
            _ => Err(yukino::ParseError::new(
                "Unexpected DatabaseValue on field crate::entities::Bar",
            )),
        }?;
        Ok(Box::new(crate::entities::Bar { float, float64 }))
    }
    fn to_raw_value(
        &self,
    ) -> Result<std::collections::HashMap<String, yukino::mapping::DatabaseValue>, yukino::ParseError>
    {
        let mut database_value = std::collections::HashMap::new();
        database_value.insert(
            "float".to_string(),
            yukino::mapping::DatabaseValue::Float(self.float),
        );
        database_value.insert(
            "float64".to_string(),
            yukino::mapping::DatabaseValue::Double(self.float64),
        );
        Ok(database_value)
    }
    fn get_definitions(&self) -> Vec<yukino::mapping::definition::TableDefinition> {
        vec![yukino::mapping::definition::TableDefinition::new(
            "bar".to_string(),
            vec![yukino::mapping::definition::IndexDefinition::new(
                "float".to_string(),
                { yukino::mapping::IndexMethod::BTree },
                vec!["float".to_string()],
                true,
            )],
            vec![
                yukino::mapping::definition::ColumnDefinition::new(
                    "__bar_id".to_string(),
                    yukino::mapping::DatabaseType::String,
                    true,
                    false,
                    true,
                ),
                yukino::mapping::definition::ColumnDefinition::new(
                    "float".to_string(),
                    yukino::mapping::DatabaseType::Float,
                    false,
                    false,
                    false,
                ),
                yukino::mapping::definition::ColumnDefinition::new(
                    "float64".to_string(),
                    yukino::mapping::DatabaseType::Double,
                    false,
                    false,
                    false,
                ),
            ],
            vec![],
        )]
    }
}
