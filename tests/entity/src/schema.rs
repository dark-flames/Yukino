#![allow(unknown_lints)]
#[derive(Clone)]
pub struct FooInner {
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
    pub fn get_int16_converter(
    ) -> yukino::resolver::field_resolver_seeds::SmallIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::SmallIntegerValueConverter::new(
            false,
            "int16".to_string(),
            "Foo".to_string(),
            "int16".to_string(),
        )
    }
    pub fn get_map_converter() -> yukino::resolver::field_resolver_seeds::MapValueConverter {
        yukino::resolver::field_resolver_seeds::MapValueConverter::new(
            "Foo".to_string(),
            "map".to_string(),
            "map".to_string(),
        )
    }
    pub fn get_list_converter() -> yukino::resolver::field_resolver_seeds::ListValueConverter {
        yukino::resolver::field_resolver_seeds::ListValueConverter::new(
            "Foo".to_string(),
            "list".to_string(),
            "list".to_string(),
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
}
impl yukino::Entity for FooInner {
    fn from_database_value(
        result: &std::collections::HashMap<String, yukino::types::DatabaseValue>,
    ) -> Result<Self, yukino::resolver::error::DataConvertError>
    where
        Self: Sized,
    {
        use yukino::resolver::ValueConverter;
        let integer = Self::get_integer_converter().to_field_value(result)?;
        let int16 = Self::get_int16_converter().to_field_value(result)?;
        let map = Self::get_map_converter().to_field_value(result)?;
        let list = Self::get_list_converter().to_field_value(result)?;
        let string = Self::get_string_converter().to_field_value(result)?;
        Ok(FooInner {
            integer,
            int16,
            map,
            list,
            string,
        })
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
        map.extend(Self::get_int16_converter().to_database_values_by_ref(&self.int16)?);
        map.extend(Self::get_map_converter().to_database_values_by_ref(&self.map)?);
        map.extend(Self::get_list_converter().to_database_values_by_ref(&self.list)?);
        map.extend(Self::get_string_converter().to_database_values_by_ref(&self.string)?);
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
                    "int16".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("int16".to_string()),
                    yukino::types::DatabaseType::SmallInteger,
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
                    "list".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("list".to_string()),
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
pub struct Foo<'t> {
    inner: std::cell::UnsafeCell<FooInner>,
    unique_id: Option<yukino::EntityUniqueID>,
    transaction: &'t yukino::Transaction,
}
impl<'t> yukino::EntityProxy<'t, FooInner> for Foo<'t> {
    fn unique_id(&self) -> Option<yukino::EntityUniqueID> {
        self.unique_id.clone()
    }
    fn set_unique_id(&mut self, unique_id: yukino::EntityUniqueID) {
        self.unique_id = Some(unique_id);
    }
    fn get_transaction(&self) -> &'t yukino::Transaction
    where
        Self: Sized,
    {
        self.transaction
    }
    fn create_proxy(inner: FooInner, transaction: &'t yukino::Transaction) -> Self
    where
        Self: Sized,
    {
        Foo {
            inner: std::cell::UnsafeCell::new(inner),
            unique_id: None,
            transaction,
        }
    }
    fn inner(&self) -> FooInner {
        self.get_inner().clone()
    }
}
impl<'t> Foo<'t> {
    pub fn get_integer(&self) -> u32 {
        let inner = self.get_inner();
        inner.integer
    }
    pub fn set_integer(&mut self, value: u32) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.integer = value;
        self
    }
    pub fn get_int16(&self) -> i16 {
        let inner = self.get_inner();
        inner.int16
    }
    pub fn set_int16(&mut self, value: i16) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.int16 = value;
        self
    }
    pub fn get_map(&self) -> &std::collections::HashMap<String, String> {
        let inner = self.get_inner();
        &inner.map
    }
    pub fn set_map(&mut self, value: std::collections::HashMap<String, String>) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.map = value;
        self
    }
    pub fn get_list(&self) -> &Vec<String> {
        let inner = self.get_inner();
        &inner.list
    }
    pub fn set_list(&mut self, value: Vec<String>) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.list = value;
        self
    }
    pub fn get_string(&self) -> &String {
        let inner = self.get_inner();
        &inner.string
    }
    pub fn set_string(&mut self, value: String) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.string = value;
        self
    }
    pub fn with_value(
        integer: u32,
        int16: i16,
        map: std::collections::HashMap<String, String>,
        list: Vec<String>,
        string: String,
    ) -> impl FnOnce() -> FooInner {
        move || FooInner {
            integer,
            int16,
            map,
            list,
            string,
        }
    }
    fn get_inner(&self) -> &FooInner {
        unsafe { self.inner.get().as_ref().unwrap() }
    }
    fn get_inner_mut(&self) -> &mut FooInner {
        unsafe { self.inner.get().as_mut().unwrap() }
    }
}
impl<'t> Drop for Foo<'t> {
    fn drop(&mut self) {
        use yukino::EntityProxy;
        self.drop_from_pool()
    }
}
