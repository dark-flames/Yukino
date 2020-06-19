use super::database_type::{DatabaseType};
use crate::mapping::error::{CompileError};
use super::helper::*;
use syn::{Type, Error};
use syn::export::TokenStream;
use crate::mapping::structure::FieldStructure;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub trait LogicalType {
    fn weight(&self) -> usize;

    fn logical_type(&self) -> &'static str;

    fn database_type(&self) -> DatabaseType;

    fn convert_to_value_token_stream(&self) -> TokenStream;

    fn convert_to_database_value_token_stream(&self) -> TokenStream;

    fn get_target_type(&self) -> Type;

    fn match_field(&self, field_structure: &FieldStructure) -> Result<bool, Error> {
        let target_type: Type = self.get_target_type();
        let field_type = &field_structure.field_type;

        cmp_type(&target_type, &field_type).map_err(
            |e| Error::new_spanned(&field_type, e.get_message())
        )
    }
}

#[allow(dead_code)]
type LogicalValueTrigger = fn(&FieldStructure) -> Result<Option<&'static str>, Error>;
#[allow(dead_code)]
pub struct ValueConverter {
     types: HashMap<&'static str, Rc<dyn LogicalType>>
}

#[allow(dead_code)]
impl ValueConverter {
    pub fn get(&self, logical_type: &'static str) -> Option<Rc<dyn LogicalType>> {
        self.types.get(logical_type).map(
            |type_ref| Rc::clone(type_ref)
        )
    }

    pub fn match_field_type(&self, field_structure: &FieldStructure) -> Option<Rc<dyn LogicalType>> {
        self.types.iter().filter(
            |(_, type_item)| {
                type_item.match_field(field_structure).unwrap_or(false)
            }
        ).fold(None, |pre, (_, type_item)| {
            if pre.is_none() || pre.as_ref().unwrap().weight() < type_item.weight() {
                Some(Rc::clone(type_item))
            } else {
                None
            }
        })
    }

    pub fn register(&mut self, logical_type: Rc<dyn LogicalType>) {
        self.types.insert(
            logical_type.logical_type(),
            logical_type
        );
    }
}

