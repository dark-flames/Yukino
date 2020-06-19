use super::database_type::{DatabaseType, DatabaseValue};
use crate::mapping::error::{CompileError};
use super::helper::*;
use syn::{Type, parse, Error};
use syn::export::TokenStream;
use std::str::FromStr;
use core::any::type_name;
use crate::mapping::structure::FieldStructure;

#[allow(dead_code)]
pub trait LogicValue {
    type ValueType;

    fn new() -> Self;

    fn logical_name() -> &'static str;

    fn database_type() -> DatabaseType;

    fn from_value(value: &Self::ValueType) -> Self;

    fn to_database_value(&self) -> DatabaseValue;

    fn match_field(field_definition: FieldStructure) -> Result<bool, Error> {
        Self::match_type(&field_definition.field_type)
    }

    fn match_type(field_type: &Type) -> Result<bool, Error> {
        let target_type: Type = parse(TokenStream::from_str(
            type_name::<Self::ValueType>()
        ).unwrap())?;

        cmp_type(&target_type, &field_type).map_err(
            |e| Error::new_spanned(&field_type, e.get_message())
        )
    }
}