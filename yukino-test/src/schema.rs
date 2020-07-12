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
                "Unexpected DatabaseValue on field crate::entities::Foo::integer",
            )),
        }?;
        let int16 = match {
            let column_name = "int16".to_string();
            result.get(&column_name)
        } {
            Some(yukino::mapping::DatabaseValue::SmallInteger(integer)) => Ok(*integer),
            _ => Err(yukino::ParseError::new(
                "Unexpected DatabaseValue on field crate::entities::Foo::int16",
            )),
        }?;
        let vec = match {
            let column_name = "vec".to_string();
            result.get(&column_name)
        } {
            Some(yukino::mapping::DatabaseValue::Json(json)) => {
                serde_json::from_value(json.clone()).map_err(|e| {
                    let message = e.to_string();
                    yukino::ParseError::new(message.as_str())
                })
            }
            _ => Err(yukino::ParseError::new(
                "Unexpected DatabaseValue on field crate::entities::Foo::vec",
            )),
        }?;
        let map = match {
            let column_name = "map".to_string();
            result.get(&column_name)
        } {
            Some(yukino::mapping::DatabaseValue::Json(json)) => {
                serde_json::from_value(json.clone()).map_err(|e| {
                    let message = e.to_string();
                    yukino::ParseError::new(message.as_str())
                })
            }
            _ => Err(yukino::ParseError::new(
                "Unexpected DatabaseValue on field crate::entities::Foo::map",
            )),
        }?;
        Ok(Box::new(crate::entities::Foo {
            integer,
            int16,
            vec,
            map,
        }))
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
        database_value.insert(
            "vec".to_string(),
            yukino::mapping::DatabaseValue::Json(serde_json::to_value(&self.vec).map_err(|e| {
                let message = e.to_string();
                yukino::ParseError::new(message.as_str())
            })?),
        );
        database_value.insert(
            "map".to_string(),
            yukino::mapping::DatabaseValue::Json(serde_json::to_value(&self.map).map_err(|e| {
                let message = e.to_string();
                yukino::ParseError::new(message.as_str())
            })?),
        );
        Ok(database_value)
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
                yukino::mapping::definition::ColumnDefinition::new(
                    "vec".to_string(),
                    yukino::mapping::DatabaseType::Json,
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
