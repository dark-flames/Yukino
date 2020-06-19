
use std::any::type_name;
use syn::{Type, parse};
use crate::mapping::structure::FieldStructure;
use crate::mapping::error::TypeError;
use super::helper::*;
use super::database_type::DatabaseType;
use syn::export::TokenStream;
use std::str::FromStr;

#[allow(dead_code)]
pub trait TypeConverter {
    type TargetType: 'static;
    type TargetDatabaseValue: 'static;

    /// overwrite it in implement
    fn match_field(field_structure: &FieldStructure) -> Result<bool, TypeError> {
        Self::match_type(&field_structure.field_type)
    }

    fn match_type(value_type: &Type) -> Result<bool, TypeError> {
        let token_stream = TokenStream::from_str(
            type_name::<Self::TargetType>()
        ).unwrap();

        let target_type: Type = parse(token_stream).unwrap();

       cmp_type(&target_type, &value_type)
    }

    fn get_logical_type() -> &'static str;

    fn get_data_base_type() -> DatabaseType;

    fn convert_to_data_base_value(value: &Self::TargetType) -> Self::TargetDatabaseValue;

    fn convert_to_rust_value(value: &Self::TargetDatabaseValue) -> Self::TargetType;
}