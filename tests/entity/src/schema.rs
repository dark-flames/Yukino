#![allow(unknown_lints)]
#[derive(Clone)]
pub struct BarInner {
    id: u64,
}
impl BarInner {
    pub fn get_id_converter(
    ) -> yukino::resolver::field_resolver_seeds::UnsignedBigIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::UnsignedBigIntegerValueConverter::new(
            true,
            "id".to_string(),
            "Bar".to_string(),
            "id".to_string(),
            yukino::types::DatabaseType::UnsignedBigInteger,
        )
    }
}
impl yukino::Entity for BarInner {
    fn from_database_value(
        result: &std::collections::HashMap<String, yukino::types::DatabaseValue>,
    ) -> Result<Self, yukino::resolver::error::DataConvertError>
    where
        Self: Sized,
    {
        use yukino::resolver::ValueConverter;
        let id = Self::get_id_converter().to_field_value(result)?;
        Ok(BarInner { id })
    }
    fn to_database_values(
        &self,
    ) -> Result<
        std::collections::HashMap<String, yukino::types::DatabaseValue>,
        yukino::resolver::error::DataConvertError,
    > {
        let mut map = std::collections::HashMap::new();
        use yukino::resolver::ValueConverter;
        map.extend(Self::get_id_converter().to_database_values_by_ref(&self.id)?);
        Ok(map)
    }
    fn get_definitions() -> Vec<yukino::definitions::TableDefinition> {
        vec![yukino::definitions::TableDefinition::new(
            "bar".to_string(),
            yukino::definitions::TableType::NormalEntityTable("Bar".to_string()),
            vec![yukino::definitions::ColumnDefinition::new(
                "id".to_string(),
                yukino::definitions::ColumnType::NormalColumn("id".to_string()),
                yukino::types::DatabaseType::UnsignedBigInteger,
                true,
                false,
                true,
                false,
            )],
            vec![],
            vec![],
        )]
    }
    fn get_field_definition(field_name: &str) -> Option<yukino::definitions::FieldDefinition> {
        match field_name {
            "id" => Some(yukino::definitions::FieldDefinition::new(
                "id".to_string(),
                "id".to_string(),
                "numeric".to_string(),
                "u64".to_string(),
                false,
                vec!["id".to_string()],
                vec![],
                None,
            )),
            _ => None,
        }
    }
    fn primary_key_values(
        &self,
    ) -> Result<
        std::collections::HashMap<String, yukino::types::DatabaseValue>,
        yukino::resolver::error::DataConvertError,
    > {
        let mut map = std::collections::HashMap::new();
        use yukino::resolver::ValueConverter;
        map.extend(Self::get_id_converter().primary_column_values_by_ref(&self.id)?);
        Ok(map)
    }
}
pub struct Bar<'t> {
    inner: std::cell::UnsafeCell<BarInner>,
    unique_id: Option<yukino::EntityUniqueID>,
    transaction: &'t yukino::Transaction,
}
impl<'t> yukino::EntityProxy<'t, BarInner> for Bar<'t> {
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
    fn create_proxy(inner: BarInner, transaction: &'t yukino::Transaction) -> Self
    where
        Self: Sized,
    {
        Bar {
            inner: std::cell::UnsafeCell::new(inner),
            unique_id: None,
            transaction,
        }
    }
    fn inner(&self) -> BarInner {
        self.get_inner().clone()
    }
}
impl<'t> Bar<'t> {
    pub fn get_id(&self) -> u64 {
        let inner = self.get_inner();
        inner.id
    }
    pub fn set_id(&mut self, value: u64) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.id = value;
        self
    }
    pub fn with_value(id: u64) -> impl FnOnce() -> BarInner {
        move || BarInner { id }
    }
    fn get_inner(&self) -> &BarInner {
        unsafe { self.inner.get().as_ref().unwrap() }
    }
    fn get_inner_mut(&self) -> &mut BarInner {
        unsafe { self.inner.get().as_mut().unwrap() }
    }
}
impl<'t> Drop for Bar<'t> {
    fn drop(&mut self) {
        use yukino::EntityProxy;
        self.drop_from_pool()
    }
}
#[derive(Clone)]
pub struct FooInner {
    integer: u32,
    int16: i16,
    list: Vec<String>,
    map: std::collections::HashMap<String, String>,
    string: String,
    boolean: bool,
    option_string: Option<String>,
    option_num: Option<u32>,
    bar: yukino::collection::AssociatedEntity<BarInner>,
}
impl FooInner {
    pub fn get_option_num_converter(
    ) -> yukino::resolver::field_resolver_seeds::UnsignedIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::UnsignedIntegerValueConverter::new(
            false,
            "option_num".to_string(),
            "Foo".to_string(),
            "option_num".to_string(),
            yukino::types::DatabaseType::UnsignedInteger,
        )
    }
    pub fn get_list_converter() -> yukino::resolver::field_resolver_seeds::ListValueConverter {
        yukino::resolver::field_resolver_seeds::ListValueConverter::new(
            "Foo".to_string(),
            "list".to_string(),
            "list".to_string(),
        )
    }
    pub fn get_integer_converter(
    ) -> yukino::resolver::field_resolver_seeds::UnsignedIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::UnsignedIntegerValueConverter::new(
            false,
            "integer".to_string(),
            "Foo".to_string(),
            "integer".to_string(),
            yukino::types::DatabaseType::UnsignedInteger,
        )
    }
    pub fn get_map_converter() -> yukino::resolver::field_resolver_seeds::MapValueConverter {
        yukino::resolver::field_resolver_seeds::MapValueConverter::new(
            "Foo".to_string(),
            "map".to_string(),
            "map".to_string(),
        )
    }
    pub fn get_option_string_converter(
    ) -> yukino::resolver::field_resolver_seeds::StringValueConverter {
        yukino::resolver::field_resolver_seeds::StringValueConverter::new(
            false,
            "Foo".to_string(),
            "option_string".to_string(),
            "option_string".to_string(),
        )
    }
    pub fn get_int16_converter(
    ) -> yukino::resolver::field_resolver_seeds::SmallIntegerValueConverter {
        yukino::resolver::field_resolver_seeds::SmallIntegerValueConverter::new(
            false,
            "int16".to_string(),
            "Foo".to_string(),
            "int16".to_string(),
            yukino::types::DatabaseType::SmallInteger,
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
    pub fn get_boolean_converter() -> yukino::resolver::field_resolver_seeds::BoolValueConverter {
        yukino::resolver::field_resolver_seeds::BoolValueConverter::new(
            false,
            "Foo".to_string(),
            "boolean".to_string(),
            "boolean".to_string(),
        )
    }
    pub fn get_bar_converter(
    ) -> yukino::resolver::field_resolver_seeds::AssociatedEntityValueConverter<BarInner> {
        yukino::resolver::field_resolver_seeds::AssociatedEntityValueConverter::new(
            "Foo".to_string(),
            "bar".to_string(),
            vec![("bar_id".to_string(), "id".to_string())]
                .into_iter()
                .collect(),
            vec![(
                "bar_id".to_string(),
                yukino::types::DatabaseType::UnsignedBigInteger,
            )]
            .into_iter()
            .collect(),
            false,
            std::marker::PhantomData::default(),
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
        let option_num = Self::get_option_num_converter().to_field_value(result)?;
        let list = Self::get_list_converter().to_field_value(result)?;
        let integer = Self::get_integer_converter().to_field_value(result)?;
        let map = Self::get_map_converter().to_field_value(result)?;
        let option_string = Self::get_option_string_converter().to_field_value(result)?;
        let int16 = Self::get_int16_converter().to_field_value(result)?;
        let string = Self::get_string_converter().to_field_value(result)?;
        let boolean = Self::get_boolean_converter().to_field_value(result)?;
        let bar = Self::get_bar_converter().to_field_value(result)?;
        Ok(FooInner {
            option_num,
            list,
            integer,
            map,
            option_string,
            int16,
            string,
            boolean,
            bar,
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
        map.extend(Self::get_option_num_converter().to_database_values_by_ref(&self.option_num)?);
        map.extend(Self::get_list_converter().to_database_values_by_ref(&self.list)?);
        map.extend(Self::get_integer_converter().to_database_values_by_ref(&self.integer)?);
        map.extend(Self::get_map_converter().to_database_values_by_ref(&self.map)?);
        map.extend(
            Self::get_option_string_converter().to_database_values_by_ref(&self.option_string)?,
        );
        map.extend(Self::get_int16_converter().to_database_values_by_ref(&self.int16)?);
        map.extend(Self::get_string_converter().to_database_values_by_ref(&self.string)?);
        map.extend(Self::get_boolean_converter().to_database_values_by_ref(&self.boolean)?);
        map.extend(Self::get_bar_converter().to_database_values_by_ref(&self.bar)?);
        Ok(map)
    }
    fn get_definitions() -> Vec<yukino::definitions::TableDefinition> {
        vec![yukino::definitions::TableDefinition::new(
            "foo".to_string(),
            yukino::definitions::TableType::NormalEntityTable("Foo".to_string()),
            vec![
                yukino::definitions::ColumnDefinition::new(
                    "option_num".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("option_num".to_string()),
                    yukino::types::DatabaseType::UnsignedInteger,
                    false,
                    false,
                    false,
                    true,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "list".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("list".to_string()),
                    yukino::types::DatabaseType::Json,
                    false,
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
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "map".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("map".to_string()),
                    yukino::types::DatabaseType::Json,
                    false,
                    false,
                    false,
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "option_string".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("Foo".to_string()),
                    yukino::types::DatabaseType::String,
                    false,
                    false,
                    false,
                    true,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "int16".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("int16".to_string()),
                    yukino::types::DatabaseType::SmallInteger,
                    false,
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
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "boolean".to_string(),
                    yukino::definitions::ColumnType::NormalColumn("Foo".to_string()),
                    yukino::types::DatabaseType::Bool,
                    false,
                    false,
                    false,
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "bar_id".to_string(),
                    yukino::definitions::ColumnType::VisualColumn,
                    yukino::types::DatabaseType::UnsignedBigInteger,
                    true,
                    false,
                    false,
                    false,
                ),
                yukino::definitions::ColumnDefinition::new(
                    "__foo_id".to_string(),
                    yukino::definitions::ColumnType::VisualColumn,
                    yukino::types::DatabaseType::String,
                    true,
                    false,
                    true,
                    false,
                ),
            ],
            vec![],
            vec![yukino::definitions::ForeignKeyDefinition::new(
                "__bar".to_string(),
                "bar".to_string(),
                vec![("bar_id".to_string(), "id".to_string())],
            )],
        )]
    }
    fn get_field_definition(field_name: &str) -> Option<yukino::definitions::FieldDefinition> {
        match field_name {
            "option_num" => Some(yukino::definitions::FieldDefinition::new(
                "option_num".to_string(),
                "option_num".to_string(),
                "numeric".to_string(),
                "u32".to_string(),
                true,
                vec!["option_num".to_string()],
                vec![],
                None,
            )),
            "list" => Some(yukino::definitions::FieldDefinition::new(
                "list".to_string(),
                "list".to_string(),
                "".to_string(),
                "Vec < String >".to_string(),
                false,
                vec!["list".to_string()],
                vec![],
                None,
            )),
            "integer" => Some(yukino::definitions::FieldDefinition::new(
                "integer".to_string(),
                "integer".to_string(),
                "numeric".to_string(),
                "u32".to_string(),
                false,
                vec!["integer".to_string()],
                vec![],
                None,
            )),
            "map" => Some(yukino::definitions::FieldDefinition::new(
                "map".to_string(),
                "map".to_string(),
                "".to_string(),
                "std :: collections :: HashMap < String , String >".to_string(),
                false,
                vec!["map".to_string()],
                vec![],
                None,
            )),
            "option_string" => Some(yukino::definitions::FieldDefinition::new(
                "option_string".to_string(),
                "option_string".to_string(),
                "string".to_string(),
                "string".to_string(),
                true,
                vec!["option_string".to_string()],
                vec![],
                None,
            )),
            "int16" => Some(yukino::definitions::FieldDefinition::new(
                "int16".to_string(),
                "int16".to_string(),
                "numeric".to_string(),
                "i16".to_string(),
                false,
                vec!["int16".to_string()],
                vec![],
                None,
            )),
            "string" => Some(yukino::definitions::FieldDefinition::new(
                "string".to_string(),
                "string".to_string(),
                "string".to_string(),
                "string".to_string(),
                false,
                vec!["string".to_string()],
                vec![],
                None,
            )),
            "boolean" => Some(yukino::definitions::FieldDefinition::new(
                "boolean".to_string(),
                "boolean".to_string(),
                "bool".to_string(),
                "string".to_string(),
                false,
                vec!["boolean".to_string()],
                vec![],
                None,
            )),
            "bar" => Some(yukino::definitions::FieldDefinition::new(
                "bar".to_string(),
                "bar".to_string(),
                "".to_string(),
                "Bar".to_string(),
                false,
                vec!["bar_id".to_string()],
                vec![],
                Some(yukino::definitions::AssociationDefinition::new(
                    "Bar".to_string(),
                    false,
                    vec![("bar_id".to_string(), "id".to_string())],
                )),
            )),
            _ => None,
        }
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
    pub fn get_option_num(&self) -> Option<u32> {
        let inner = self.get_inner();
        inner.option_num
    }
    pub fn set_option_num(&mut self, value: u32) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.option_num = Some(value);
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
    pub fn get_integer(&self) -> u32 {
        let inner = self.get_inner();
        inner.integer
    }
    pub fn set_integer(&mut self, value: u32) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.integer = value;
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
    pub fn get_option_string(&self) -> &Option<String> {
        let inner = self.get_inner();
        &inner.option_string
    }
    pub fn set_option_string(&mut self, value: String) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.option_string = Some(value);
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
    pub fn get_string(&self) -> &String {
        let inner = self.get_inner();
        &inner.string
    }
    pub fn set_string(&mut self, value: String) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.string = value;
        self
    }
    pub fn get_boolean(&self) -> &bool {
        let inner = self.get_inner();
        &inner.boolean
    }
    pub fn set_boolean(&mut self, value: bool) -> &mut Self {
        let inner = self.get_inner_mut();
        inner.boolean = value;
        self
    }
    pub fn get_bar(&self) -> Bar {
        use yukino::EntityProxy;
        let inner = self.get_inner();
        if let yukino::collection::AssociatedEntity::Unresolved(values) = &inner.bar {
            let mut_inner = self.get_inner_mut();
            let result = self
                .get_transaction()
                .get_repository()
                .find(values)
                .unwrap();
            mut_inner.bar = yukino::collection::AssociatedEntity::Resolved(result)
        }
        let entity = inner.bar.get().unwrap().clone();
        self.get_transaction().create_entity(move || entity)
    }
    pub fn set_bar(&mut self, value: Bar) -> &mut Self {
        use yukino::EntityProxy;
        let mut_inner = self.get_inner_mut();
        mut_inner.bar = yukino::collection::AssociatedEntity::Resolved(value.inner());
        self
    }
    pub fn with_value(
        option_num: Option<u32>,
        list: Vec<String>,
        integer: u32,
        map: std::collections::HashMap<String, String>,
        option_string: Option<String>,
        int16: i16,
        string: String,
        boolean: bool,
        bar: yukino::collection::AssociatedEntity<BarInner>,
    ) -> impl FnOnce() -> FooInner {
        move || FooInner {
            option_num,
            list,
            integer,
            map,
            option_string,
            int16,
            string,
            boolean,
            bar,
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
