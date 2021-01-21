#![allow(unknown_lints)]
pub struct FooInner {
    integer: u32,
    int16: i16,
    list: Vec<String>,
    map: std::collections::HashMap<String, String>,
    string: String,
}
impl FooInner {
    pub fn get_map_converter() -> yukino::resolver::field_resolver_seeds::MapValueConverter {
        yukino::resolver::field_resolver_seeds::MapValueConverter::new(
            "Foo".to_string(),
            "map".to_string(),
            "map".to_string(),
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
    pub fn get_int16_converter(
    ) -> yukino::resolver::field_resolver_seeds::SmallIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::SmallIntegerValueConverter::new(
            false,
            "int16".to_string(),
            "Foo".to_string(),
            "int16".to_string(),
        )
    }
    pub fn get_integer_converter(
    ) -> yukino::resolver::field_resolver_seeds::UnsignedIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::UnsignedIntegerValueConverter::new(
            false,
            "integer".to_string(),
            "Foo".to_string(),
            "integer".to_string(),
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
        let map = Self::get_map_converter().to_field_value(result)?;
        let string = Self::get_string_converter().to_field_value(result)?;
        let int16 = Self::get_int16_converter().to_field_value(result)?;
        let integer = Self::get_integer_converter().to_field_value(result)?;
        let list = Self::get_list_converter().to_field_value(result)?;
        Ok(Box::new(FooInner {
            map,
            string,
            int16,
            integer,
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
        map.extend(Self::get_map_converter().to_database_values_by_ref(&self.map)?);
        map.extend(Self::get_string_converter().to_database_values_by_ref(&self.string)?);
        map.extend(Self::get_int16_converter().to_database_values_by_ref(&self.int16)?);
        map.extend(Self::get_integer_converter().to_database_values_by_ref(&self.integer)?);
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
                    "map".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("map".to_string()),
                    yukino::types::DatabaseType::Json,
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
                    "int16".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("int16".to_string()),
                    yukino::types::DatabaseType::SmallInteger,
                    false,
                    false,
                    false,
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
pub struct Foo<'r> {
    inner: FooInner,
    unique_id: Option<yukino::EntityUniqueID>,
    repository: &'r yukino::repository::Repository<'r, Self, FooInner>,
}
impl<'r> yukino::EntityProxy<'r, FooInner> for Foo<'r> {
    fn unique_id(&self) -> Option<yukino::EntityUniqueID> {
        self.unique_id.clone()
    }
    fn set_unique_id(&mut self, unique_id: yukino::EntityUniqueID) {
        self.unique_id = Some(unique_id);
    }
    fn get_repository(&self) -> &'r yukino::repository::Repository<'r, Self, FooInner>
    where
        Self: Sized,
    {
        self.repository
    }
    fn create_proxy(
        inner: FooInner,
        repository: &'r yukino::repository::Repository<'r, Self, FooInner>,
    ) -> Self
    where
        Self: Sized,
    {
        Foo {
            inner,
            unique_id: None,
            repository,
        }
    }
}
impl<'r> Foo<'r> {
    pub fn get_map(&self) -> &std::collections::HashMap<String, String> {
        &self.inner.map
    }
    pub fn set_map(&mut self, value: std::collections::HashMap<String, String>) -> &mut Self {
        self.inner.map = value;
        self
    }
    pub fn get_string(&self) -> &String {
        &self.inner.string
    }
    pub fn set_string(&mut self, value: String) -> &mut Self {
        self.inner.string = value;
        self
    }
    pub fn get_int16(&self) -> i16 {
        self.inner.int16
    }
    pub fn set_int16(&mut self, value: i16) -> &mut Self {
        self.inner.int16 = value;
        self
    }
    pub fn get_integer(&self) -> u32 {
        self.inner.integer
    }
    pub fn set_integer(&mut self, value: u32) -> &mut Self {
        self.inner.integer = value;
        self
    }
    pub fn get_list(&self) -> &Vec<String> {
        &self.inner.list
    }
    pub fn set_list(&mut self, value: Vec<String>) -> &mut Self {
        self.inner.list = value;
        self
    }
    pub fn with_value(
        map: std::collections::HashMap<String, String>,
        string: String,
        int16: i16,
        integer: u32,
        list: Vec<String>,
    ) -> impl FnOnce() -> FooInner {
        move || FooInner {
            map,
            string,
            int16,
            integer,
            list,
        }
    }
}
impl<'r> Drop for Foo<'r> {
    fn drop(&mut self) {
        use yukino::EntityProxy;
        self.drop_from_pool()
    }
}
